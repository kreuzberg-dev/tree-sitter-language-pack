const baseuml = require("./grammer/baseuml");
const jsonuml = require("./grammer/jsonuml");
const yamluml = require("./grammer/yamluml");
module.exports = grammar({
  name: "plantuml",
  conflicts: ($) => [
    // NOTE:# is also key in html mark
    // also mark the color,
    // so they must be conflicts
    [$.colorleader, $.uniqkey],
    // NOTE: * is uniqkey, also string mark
    [$.block_1, $._command_unit],
    // NOTE: style block is conflicts with uniqkey, also use "<" and
    // ">" and "/"
    [$.block_style, $.uniqkey],
    // NOTE: it is because $
    [$.message, $.uniquevar],
  ],

  rules: {
    plantuml: ($) => choice($.baseuml, $.gannttuml, $.saltuml, $.mindmapuml, $.yamluml, $.jsonuml),
    ...yamluml,
    ...baseuml,
    ...jsonuml,
    command: ($) => seq(repeat1($._command_unit), "\n"),
    _command_unit: ($) =>
      choice($.message, $.block, $.string, $.method, $.uniqkey, $.color, $.identifier, $.uniquevar, $.comment),
    method: ($) => seq($.identifier, "()"),
    message: ($) => seq("$", $.identifier, "(", optional($.string), ")"),
    block: ($) => choice($.block_1, $._block_2, $._block_3, $.block_style),
    block_1: ($) => seq(optional(choice($.uniquevar, $.identifier)), "{", repeat($.command), "}"),
    _block_2: ($) => seq("[", repeat($._command_unit), "]"),
    _block_3: ($) => seq("(", repeat($._command_unit), ")"),
    block_style: ($) =>
      seq(
        "<",
        field("head", seq($.identifier, optional(seq(":", $.identifier)), ">")),
        choice(repeat($.command), repeat($._command_unit)),
        field("end", seq("</", $.identifier, ">")),
      ),
    uniqkey: (_$) => choice(".", ">", "<", "_", ":", "*", "+", "-", "#", "!", "/", ",", "|", "=", ";"),
    string: ($) => seq('"', optional(repeat(choice(token.immediate(prec(1, /[^"\\]+/)), $.escape_sequence))), '"'),
    escape_sequence: (_$) =>
      token.immediate(
        seq("\\", choice(/[^xu0-7]/, /[0-7]{1,3}/, /x[0-9a-fA-F]{2}/, /u[0-9a-fA-F]{4}/, /u\{[0-9a-fA-F]+\}/)),
      ),
    color: ($) => seq($.colorleader, $.identifier),
    colorleader: (_$) => "#",
    uniquevar: ($) => seq("$", $.identifier),
    identifier: (_$) => /[a-zA-Z0-9_./]+/,
    // TODO: multiline comment
    comment: ($) => choice($._signallinecomment, $._mutilinecomment),
    _signallinecomment: (_$) => token(seq("'", /[^\n]+/g)),

    _mutilinecomment: (_$) => seq("/'", repeat(/[^\n^']+/g), "'/"),
  },
});
