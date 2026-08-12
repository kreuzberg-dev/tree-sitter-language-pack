/* globals grammar seq choice repeat repeat1 optional field token */
module.exports = grammar({
  name: "yul",
  extras: ($) => [$.comment, /[\s\uFEFF\u2060\u200B\u00A0]/],
  rules: {
    // TODO: add the actual grammar rules
    source_file: ($) => repeat1($.object),

    object: ($) =>
      seq(
        "object",
        field("name", $.stringLiteral),
        field("body", seq("{", $.code, repeat(choice($.data, $.object)), "}")),
      ),

    code: ($) => seq("code", field("body", $.block)),
    data: ($) => seq("data", field("key", $.stringLiteral), field("value", choice($.hexLiteral, $.stringLiteral))),

    block: ($) => seq("{", repeat($.statement), "}"),

    statement: ($) =>
      choice(
        $.block,
        $.functionDefinition,
        $.variableDeclaration,
        $.assignment,
        $.if,
        $.expression,
        $.switch,
        $.forLoop,
        $.breakContinue,
        $.leave,
      ),

    functionDefinition: ($) =>
      seq(
        "function",
        field("name", $.identifier),
        "(",
        field("parameters", optional($.typedIdentifierList)),
        ")",
        optional(seq("->", field("returns", $.typedIdentifierList))),
      ),
    variableDeclaration: ($) =>
      seq("let", field("left", $.typedIdentifierList), optional(seq(":=", field("right", $.expression)))),
    assignment: ($) => seq(field("left", $.identifierList), ":=", field("right", $.expression)),
    expression: ($) => choice($.functionCall, $.identifier, $.literal),
    if: ($) => seq("if", $.expression, $.block),
    switch: ($) =>
      seq(
        "switch",
        field("value", $.expression),
        field("body", choice(seq(repeat1($.case), optional($.default)), $.default)),
      ),
    case: ($) => seq("case", field("value", $.literal), $.block),
    default: ($) => seq("default", $.block),
    forLoop: ($) => seq("for", $.block, $.expression, $.block, $.block),
    breakContinue: (_$) => choice("break", "continue"),
    leave: (_$) => "leave",
    functionCall: ($) =>
      seq(field("name", $.identifier), "(", optional(seq($.expression, repeat(seq(",", $.expression)))), ")"),
    identifier: (_$) => /[a-zA-Z_$][a-zA-Z_$0-9.]*/,
    identifierList: ($) => seq($.identifier, repeat(seq(",", $.identifier))),
    typeName: ($) => $.identifier,
    typedIdentifierList: ($) =>
      seq($.identifier, optional(seq(":", $.typeName)), repeat(seq(",", $.identifier, optional(seq(":", $.typeName))))),
    literal: ($) =>
      seq(
        choice($.numberLiteral, $.stringLiteral, $.hexLiteral, $.trueLiteral, $.falseLiteral),
        optional(seq(":", $.typeName)),
      ),
    numberLiteral: ($) => choice($.hexNumber, $.decimalNumber),
    hexLiteral: (_$) => /hex("([0-9a-fA-F]{2})*"|'([0-9a-fA-F]{2})*')/,
    stringLiteral: (_$) => /"([^"\r\n\\]|\\.)*"/,
    trueLiteral: (_$) => "true",
    falseLiteral: (_$) => "false",
    hexNumber: (_$) => /0x[0-9a-fA-F]+/,
    decimalNumber: (_$) => /[0-9]+/,
    comment: (_$) => token(choice(seq("//", /.*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"))),
  },
});
