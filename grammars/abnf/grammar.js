num_val_template = (indicator, char_rule) =>
  seq(
    indicator,
    repeat1(char_rule),
    optional(choice(repeat1(seq(".", repeat1(char_rule))), seq("-", repeat1(char_rule)))),
  );

module.exports = grammar({
  name: "abnf",

  extras: (_$) => [],

  conflicts: ($) => [[$.concatenation], [$.alternation]],

  rules: {
    source_file: ($) => $.rulelist,

    rulelist: ($) => repeat1(choice($.rule, prec(1, seq(repeat($._c_wsp), $._c_nl)))),

    rule: ($) => seq($.rulename, $.defined_as, $.elements, $._c_nl),

    rulename: (_$) => /[A-Za-z][A-Za-z0-9-]*/,

    defined_as: ($) => seq(repeat($._c_wsp), choice("=", "=/"), repeat($._c_wsp)),

    elements: ($) => prec.left(1, seq($.alternation, repeat($._c_wsp))),

    _c_wsp: ($) =>
      choice(
        $._WSP,

        seq($._c_nl, $._WSP),
      ),

    _c_nl: ($) => choice($.comment, $._CRLF),

    comment: (_$) => /;[ \t\x21-\x7E]*\r\n/,

    alternation: ($) => seq($.concatenation, repeat(seq(repeat($._c_wsp), "/", repeat($._c_wsp), $.concatenation))),

    concatenation: ($) => seq($.repetition, repeat(seq(repeat1($._c_wsp), $.repetition))),

    repetition: ($) => seq(optional($.repeat), $.element),

    repeat: ($) => choice(repeat1($.DIGIT), seq(repeat($.DIGIT), "*", repeat($.DIGIT))),

    // NB: $.core_rulename isn't explicitly defined in RFC 5234.
    element: ($) => choice($.core_rulename, $.rulename, $.group, $.option, $.char_val, $.num_val, $.prose_val),

    group: ($) => seq("(", repeat($._c_wsp), $.alternation, repeat($._c_wsp), ")"),

    option: ($) => seq("[", repeat($._c_wsp), $.alternation, repeat($._c_wsp), "]"),

    // Updated by RFC 7405
    char_val: ($) => choice($.case_insensitive_string, $.case_sensitive_string),

    case_insensitive_string: ($) => seq(optional("%i"), $.quoted_string),

    case_sensitive_string: ($) => seq("%s", $.quoted_string),

    quoted_string: ($) => seq($._DQUOTE, /[\x20-\x21\x23-\x7E]*/, $._DQUOTE),

    num_val: ($) => seq("%", choice($.bin_val, $.dec_val, $.hex_val)),

    bin_val: ($) => num_val_template("b", $.BIT),

    dec_val: ($) => num_val_template("d", $.DIGIT),

    hex_val: ($) => num_val_template("x", $.HEXDIG),

    prose_val: (_$) => seq("<", /[\x20-\x3D\x3F-\x7E]*/, ">"),

    // RFC 5234 doesn't define these as special rulenames; they just
    // happen to match the rulename rule. However, Appendix B defines
    // commonly used rules which use these rulenames and they are
    // implicitly used in many ABNF grammars.
    core_rulename: (_$) =>
      choice(
        "ALPHA",
        "BIT",
        "CHAR",
        "CR",
        "CRLF",
        "CTL",
        "DIGIT",
        "DQUOTE",
        "HEXDIG",
        "HTAB",
        "LF",
        "LWSP",
        "OCTET",
        "SP",
        "VCHAR",
        "WSP",
      ),

    ALPHA: (_$) => /[A-Za-z]/,

    BIT: (_$) => choice("0", "1"),

    DIGIT: (_$) => /[0-9]/,

    _CR: (_$) => "\r",

    _CRLF: ($) => seq($._CR, $._LF),

    _DQUOTE: (_$) => '"',

    // RFC 5234 only defines upper-case HEXDIGs, but this grammar is
    // more lenient.
    HEXDIG: (_$) => /[0-9A-Fa-f]/,

    _HTAB: (_$) => "\t",

    _LF: (_$) => "\n",

    _SP: (_$) => " ",

    VCHAR: (_$) => /[\x21-\x7E]/,

    _WSP: ($) => choice($._SP, $._HTAB),
  },
});
