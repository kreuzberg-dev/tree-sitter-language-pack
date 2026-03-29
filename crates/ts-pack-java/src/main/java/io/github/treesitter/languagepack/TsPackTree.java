package io.github.treesitter.languagepack;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

/**
 * Wraps an opaque {@code TsPackTree*} handle representing a parsed syntax tree.
 *
 * <p>Implements {@link AutoCloseable} so it can be used in try-with-resources blocks:
 *
 * <pre>{@code
 * try (var registry = new TsPackRegistry()) {
 *     try (var tree = registry.parseString("python", "def hello(): pass")) {
 *         System.out.println(tree.rootNodeType());    // "module"
 *         System.out.println(tree.rootChildCount());   // 1
 *         System.out.println(tree.containsNodeType("function_definition")); // true
 *     }
 * }
 * }</pre>
 *
 * <p>This class is <strong>not</strong> thread-safe.
 */
public class TsPackTree implements AutoCloseable {

  private static final Linker LINKER = Linker.nativeLinker();
  private static final SymbolLookup LOOKUP;

  private static final MethodHandle TREE_FREE;
  private static final MethodHandle TREE_ROOT_NODE_TYPE;
  private static final MethodHandle TREE_ROOT_CHILD_COUNT;
  private static final MethodHandle TREE_CONTAINS_NODE_TYPE;
  private static final MethodHandle TREE_HAS_ERROR_NODES;
  private static final MethodHandle TREE_TO_SEXP;
  private static final MethodHandle TREE_ERROR_COUNT;
  private static final MethodHandle FREE_STRING;

  static {
    String libPath = System.getenv("TSPACK_LIB_PATH");
    if (libPath != null && !libPath.isEmpty()) {
      LOOKUP = SymbolLookup.libraryLookup(java.nio.file.Path.of(libPath), Arena.global());
    } else {
      LOOKUP = SymbolLookup.libraryLookup("ts_pack_ffi", Arena.global());
    }

    TREE_FREE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

    TREE_ROOT_NODE_TYPE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_root_node_type").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    TREE_ROOT_CHILD_COUNT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_root_child_count").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

    TREE_CONTAINS_NODE_TYPE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_contains_node_type").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    TREE_HAS_ERROR_NODES =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_has_error_nodes").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS));

    // ts_pack_tree_to_sexp(pointer) -> pointer
    TREE_TO_SEXP =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_to_sexp").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_tree_error_count(pointer) -> long (usize)
    TREE_ERROR_COUNT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_tree_error_count").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS));

    FREE_STRING =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_free_string").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
  }

  private MemorySegment treePtr;

  /** Package-private constructor. Use {@link TsPackRegistry#parseString(String, String)}. */
  TsPackTree(MemorySegment treePtr) {
    this.treePtr = treePtr;
  }

  /** Frees the underlying native tree. Safe to call multiple times. */
  @Override
  public void close() {
    if (treePtr != null && !treePtr.equals(MemorySegment.NULL)) {
      try {
        TREE_FREE.invokeExact(treePtr);
      } catch (Throwable t) {
        throw new RuntimeException("Failed to invoke ts_pack_tree_free", t);
      }
      treePtr = MemorySegment.NULL;
    }
  }

  /**
   * Returns the type name of the root node.
   *
   * @return the root node type (e.g. {@code "module"} for Python)
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public String rootNodeType() {
    ensureOpen();
    try {
      MemorySegment cStr = (MemorySegment) TREE_ROOT_NODE_TYPE.invokeExact(treePtr);
      if (cStr.equals(MemorySegment.NULL)) {
        throw new RuntimeException("ts_pack_tree_root_node_type returned null");
      }
      try {
        return cStr.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(cStr);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_root_node_type", t);
    }
  }

  /**
   * Returns the number of named children of the root node.
   *
   * @return the child count (non-negative)
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public int rootChildCount() {
    ensureOpen();
    try {
      return (int) TREE_ROOT_CHILD_COUNT.invokeExact(treePtr);
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_root_child_count", t);
    }
  }

  /**
   * Checks whether any node in the tree has the given type name.
   *
   * @param nodeType the node type to search for (e.g. {@code "function_definition"})
   * @return {@code true} if a matching node exists
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public boolean containsNodeType(String nodeType) {
    ensureOpen();
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cType = arena.allocateFrom(nodeType);
      return (boolean) TREE_CONTAINS_NODE_TYPE.invokeExact(treePtr, cType);
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_contains_node_type", t);
    }
  }

  /**
   * Checks whether the tree contains any ERROR or MISSING nodes.
   *
   * @return {@code true} if error nodes are present
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public boolean hasErrorNodes() {
    ensureOpen();
    try {
      return (boolean) TREE_HAS_ERROR_NODES.invokeExact(treePtr);
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_has_error_nodes", t);
    }
  }

  /**
   * Returns the S-expression representation of the syntax tree.
   *
   * @return the S-expression string
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public String toSexp() {
    ensureOpen();
    try {
      MemorySegment cStr = (MemorySegment) TREE_TO_SEXP.invokeExact(treePtr);
      if (cStr.equals(MemorySegment.NULL)) {
        throw new RuntimeException("ts_pack_tree_to_sexp returned null");
      }
      try {
        return cStr.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(cStr);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_to_sexp", t);
    }
  }

  /**
   * Returns the count of ERROR and MISSING nodes in the tree.
   *
   * @return the error node count (non-negative)
   * @throws IllegalStateException if the tree has been closed
   * @throws RuntimeException if the native call fails
   */
  public long errorCount() {
    ensureOpen();
    try {
      return (long) TREE_ERROR_COUNT.invokeExact(treePtr);
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_tree_error_count", t);
    }
  }

  private void ensureOpen() {
    if (treePtr == null || treePtr.equals(MemorySegment.NULL)) {
      throw new IllegalStateException("Tree has been closed");
    }
  }
}
