package io.xberg.tslp.contracttests;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import java.io.Closeable;
import java.lang.reflect.Method;
import java.util.Locale;
import java.util.Set;
import org.junit.jupiter.api.Test;

/**
 * Guards the JVM half of alef.toml's capsule-ownership contract:
 *
 * <pre>
 * [crates.java.capsule_types.Language]
 * pointer_ownership = "borrowed_static"
 * abi_compatible = true
 * host_destructor = "none"
 *
 * [crates.kotlin_android.capsule_types.Language]
 * pointer_ownership = "borrowed_static"
 * abi_compatible = true
 * host_destructor = "none"
 * </pre>
 *
 * <p>Both declarations assert that the host-language {@code Language} wrapper never frees the
 * native pointer it wraps — alef therefore emits no destructor call when constructing one of these
 * wrappers from a raw pointer. That assertion is only true because, as measured against the pinned
 * versions below, neither wrapper class implements {@link AutoCloseable}/{@link Closeable} nor
 * declares a close/finalize/free/dispose method of its own.
 *
 * <p>Nothing else in this repository re-checks that fact. Bumping {@code package_version} in
 * alef.toml — a one-line edit — silently invalidates it if a newer wrapper release adds a
 * destructor. This test is the tripwire: it inspects the actual classes on the test classpath
 * (pinned in {@code pom.xml} to the same versions as alef.toml) and fails loudly if either
 * wrapper's shape has changed.
 *
 * <p>Two controls keep a vacuous pass from reading as a real one. {@code jtreesitter}'s
 * {@code Parser} IS {@link AutoCloseable}, so it is an in-library control. {@code ktreesitter} has
 * no such control available: measured against {@code ktreesitter-jvm:0.25.1}, not one of its 62
 * classes declares close/finalize/free/dispose. Its control is therefore
 * {@link #detectorRejectsAKnownDestructorBearingClass()} (the detector must actually fire) plus
 * {@link #ktreesitterLanguageIsTheRealClass()} (we are inspecting the real class, not a stub).
 *
 * <p>The pin in alef.toml is the Android artifact, not {@code -jvm}. Their {@code Language} classes
 * were compared with {@code javap} at 0.25.1 and are shape-identical — same 9 fields, same 29
 * methods, no destructor in either — so the classpath-friendly {@code -jvm} artifact is a faithful
 * proxy for the contract. They do NOT agree on {@code Parser}: the Android build implements
 * {@link AutoCloseable}, the {@code -jvm} build does not. That is why {@code Parser} cannot serve
 * as the ktreesitter control here. ~keep
 */
final class CapsuleOwnershipContractTest {

    private static final Set<String> DESTRUCTOR_LIKE_METHOD_NAMES =
            Set.of("close", "finalize", "free", "dispose");

    @Test
    void jtreesitterLanguageOwnsNoNativeResource() throws ClassNotFoundException {
        assertNoDestructor("io.github.treesitter.jtreesitter.Language");
    }

    @Test
    void jtreesitterParserIsAutoCloseable_positiveControl() throws ClassNotFoundException {
        assertIsAutoCloseable("io.github.treesitter.jtreesitter.Parser");
    }

    @Test
    void ktreesitterLanguageOwnsNoNativeResource() throws ClassNotFoundException {
        assertNoDestructor("io.github.treesitter.ktreesitter.Language");
    }

    /**
     * The detector must actually fire on a class that has a destructor, or every
     * {@code assertNoDestructor} above passes for the wrong reason. {@link java.io.FileInputStream}
     * implements {@link Closeable} and declares {@code close()}, so it exercises both arms.
     */
    @Test
    void detectorRejectsAKnownDestructorBearingClass() {
        assertThrows(
                AssertionError.class,
                () -> assertNoDestructor("java.io.FileInputStream"),
                "assertNoDestructor accepted a class that implements Closeable and declares"
                        + " close() — the detector does not discriminate, so every contract"
                        + " assertion in this file is vacuous.");
    }

    /**
     * Guards against inspecting a stub or an unrelated class of the same name: the real
     * {@code ktreesitter} {@code Language} declares the native accessors the capsule contract is
     * about. ktreesitter has no destructor-bearing class of its own to use as a control, so this
     * pairs with {@link #detectorRejectsAKnownDestructorBearingClass()} to cover it.
     */
    @Test
    void ktreesitterLanguageIsTheRealClass() throws ClassNotFoundException {
        Class<?> type = loadWithoutInitializing("io.github.treesitter.ktreesitter.Language");
        assertTrue(
                declaresMethod(type, "symbolName") && declaresMethod(type, "fieldIdForName"),
                "ktreesitter Language does not declare the native accessors it is expected to;"
                        + " the class on the test classpath is not the one the contract measured,"
                        + " so its clean destructor scan proves nothing.");
    }

    private static boolean declaresMethod(Class<?> type, String name) {
        for (Method method : type.getDeclaredMethods()) {
            if (method.getName().equals(name)) {
                return true;
            }
        }
        return false;
    }

    /**
     * Asserts that {@code className} neither implements {@link AutoCloseable}/{@link Closeable}
     * nor declares its own close/finalize/free/dispose method. Loaded without initialization
     * ({@code Class.forName(name, false, loader)}) so no native linking or static init runs —
     * this test inspects shape only, it never constructs an instance.
     */
    private static void assertNoDestructor(String className) throws ClassNotFoundException {
        Class<?> type = loadWithoutInitializing(className);

        assertFalse(
                AutoCloseable.class.isAssignableFrom(type),
                className + " now implements AutoCloseable — alef.toml declares"
                        + " host_destructor = \"none\" for this capsule type; update the contract"
                        + " (and the generated destructor wiring) before bumping the pin.");
        assertFalse(
                Closeable.class.isAssignableFrom(type),
                className + " now implements Closeable — alef.toml declares"
                        + " host_destructor = \"none\" for this capsule type; update the contract"
                        + " (and the generated destructor wiring) before bumping the pin.");

        for (Method method : type.getDeclaredMethods()) {
            String lowerName = method.getName().toLowerCase(Locale.ROOT);
            if (DESTRUCTOR_LIKE_METHOD_NAMES.contains(lowerName)) {
                fail(className + " now declares " + method + " — this reads as a destructor;"
                        + " alef.toml's host_destructor = \"none\" pin for this capsule type is"
                        + " no longer accurate and must be revisited before bumping the pin.");
            }
        }
    }

    private static void assertIsAutoCloseable(String className) throws ClassNotFoundException {
        Class<?> type = loadWithoutInitializing(className);
        assertTrue(
                AutoCloseable.class.isAssignableFrom(type),
                className + " is expected to be AutoCloseable (positive control); if this fails,"
                        + " the inspection itself is broken, not just the contract.");
    }

    private static Class<?> loadWithoutInitializing(String className) throws ClassNotFoundException {
        return Class.forName(className, false, CapsuleOwnershipContractTest.class.getClassLoader());
    }
}
