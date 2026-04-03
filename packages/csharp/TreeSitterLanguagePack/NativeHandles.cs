using System;
using System.Runtime.InteropServices;

namespace TreeSitterLanguagePack;

/// <summary>Opaque handle to a tree-sitter Tree. Passed by value across FFI.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct Tree
{
    internal IntPtr Handle;
}

/// <summary>Opaque handle to a tree-sitter Language. Passed by value across FFI.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct Language
{
    internal IntPtr Handle;
}

/// <summary>Opaque handle to a tree-sitter Parser. Passed by value across FFI.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct Parser
{
    internal IntPtr Handle;
}

/// <summary>Opaque handle to a language registry. Passed by value across FFI.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct LanguageRegistry
{
    internal IntPtr Handle;
}
