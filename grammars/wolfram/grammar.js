// @ts-check
/// <reference path="./node_modules/tree-sitter-cli/dsl.d.ts" />

/** @type RuleBuilder<any> */
const EXTERNAL = (() => {
  /** @type any */
  const tmp = Symbol("EXTERNAL");
  return tmp;
})();

/** @type Rule */
const SYMB_KEEP = (() => {
  /** @type any */
  const tmp = Symbol("SYMB_KEEP");
  return tmp;
})();

/** @type Rule */
const EXPR_KEEP = (() => {
  /** @type any */
  const tmp = Symbol("EXPR_KEEP");
  return tmp;
})();

/** @type Rule */
const NULLABLE_EXPR_KEEP = (() => {
  /** @type any */
  const tmp = Symbol("NULLABLE_EXPR_KEEP");
  return tmp;
})();

/** @type Rule */
const EXPR1_KEEP = (() => {
  /** @type any */
  const tmp = Symbol("EXPR1_KEEP");
  return tmp;
})();

/** @type Rule */
const EXPR1_UNARY_KEEP = (() => {
  /** @type any */
  const tmp = Symbol("EXPR1_UNARY_KEEP");
  return tmp;
})();

const defaultReplacer = (/** @type any */ e) => e;
let globalReplacer = defaultReplacer;

/** @type typeof grammar */
const grammar0 = (/** @type any */ e) => {
  return grammar({
    ...e,
    rules: Object.fromEntries(Object.entries(e.rules).filter(([, v]) => v !== EXTERNAL)),
  });
};

/* eslint-disable no-unused-vars */
/** @type typeof seq */
const seq0 = (/** @type any */ ...args) => {
  return seq(...args.map((/** @type any */ e) => globalReplacer(e)));
};

/** @type typeof optional */
const optional0 = (/** @type any */ rule) => {
  return optional(globalReplacer(rule));
};

/** @type typeof choice */
const choice0 = (/** @type any */ ...args) => {
  return choice(...args.map((/** @type any */ e) => globalReplacer(e)));
};
/* eslint-enable no-unused-vars */

const REP = Symbol("rep");
// https://reference.wolfram.com/language/tutorial/OperatorInputForms.html
const PREC_ORIG = [
  {
    MESSAGE_NAME_REP: REP,
  },
  {
    SHARP: prec,
    PERCENT: prec,
    UNDERSCORE: prec,
  },
  {
    GET: prec,
  },
  {
    UNDEROVERSCRIPT: prec,
  },
  {
    OVERUNDERSCRIPT: prec,
  },
  {
    OVERSCRIPT: prec.right,
  },
  {
    UNDERSCRIPT: prec.right,
  },
  //
  {
    SUBSCRIPT_POWER: prec,
  },
  {
    SUBSCRIPT: prec.right,
  },
  //
  {
    // No precedence. 3 or more operands are not allowed.
    PATTERN_TEST: prec.left,
  },
  {
    APPLY: prec,
    PART: prec,
  },
  {
    INCREMENT: prec,
    DECREMENT: prec,
  },
  {
    PRE_INCREMENT: prec,
    PRE_DECREMENT: prec,
  },
  {
    COMPOSITION_REP: REP,
  },
  {
    RIGHT_COMPOSITION_REP: REP,
  },
  {
    APPLICATION: prec.left,
  },
  {
    APPLY_AT: prec.right,
  },
  {
    TILDE: prec.left,
  },
  {
    MAP: prec.right,
    MAP_ALL: prec.right,
    APPLY_ATAT: prec.right,
    APPLY_ATATAT: prec.right,
  },
  {
    FACTORIAL: prec,
    FACTORIAL2: prec,
  },
  {
    DERIVATIVE: prec,
  },
  {
    STRING_JOIN_REP: REP,
  },
  {
    POWER: prec.right,
    POWER_SUBSCRIPT: prec.left,
  },
  // <SQRT> split by technical reason
  {
    SQRT: prec,
  },
  {
    ANY_ROOT: prec,
  },
  // </SQRT>
  {
    DIFFERENTIAL_D: prec,
    DEL: prec,
  },
  {
    SQUARE: prec,
    SMALL_CIRCLE_REP: REP,
  },
  {
    CIRCLE_DOT_REP: REP,
  },
  {
    NON_COMMUTATIVE_MULTIPLY_REP: REP,
  },
  {
    CROSS_REP: REP,
  },
  {
    DOT_REP: REP,
  },
  {
    NEGATIVE: prec,
    POSITIVE: prec,
    NEGATIVE_POSITIVE: prec,
  },
  {
    PER: prec.left,
    DIVIDE: prec.left,
    DIVIDE_BOX: prec.left,
  },
  {
    BACKSLASH_REP: REP,
  },
  {
    DIAMOND_REP: REP,
  },
  {
    WEDGE_REP: REP,
  },
  {
    VEE_REP: REP,
  },
  {
    CIRCLE_TIMES_REP: REP,
  },
  {
    CENTER_DOT_REP: REP,
  },
  {
    TIMES_AST_REP: REP,
    TIMES_CROSS_REP: REP,
  },
  {
    STAR_REP: REP,
  },
  {
    VERTICAL_TILDE_REP: REP,
  },
  {
    COPRODUCT_REP: REP,
  },
  {
    CAP_REP: REP,
  },
  {
    CUP_REP: REP,
  },
  {
    CIRCLE_PLUS_REP: REP,
    CIRCLE_MINUS: prec.left,
  },
  {
    INTEGRATE: prec,
  },
  {
    PLUS_REP: REP,
    MINUS_REP: REP,
    PLUS_MINUS: prec.left,
    MINUS_PLUS: prec.left,
  },
  {
    NEGATIVE: prec,
    POSITIVE: prec,
    NEGATIVE_POSITIVE: prec,
  },
  {
    INTERSECTION_REP: REP,
  },
  {
    UNION_REP: REP,
  },
  {
    SPAN_REP: REP,
  },
  {
    EQUAL_REP: REP,
    UNEQUAL_REP: REP,
    EQUAL_ASCII_REP: REP,
    UNEQUAL_ASCII_REP: REP,
    GREATER_REP: REP,
    GREATER_EQUAL_REP: REP,
    GREATER_ASCII_REP: REP,
    GREATER_EQUAL_ASCII_REP: REP,
    LESS_REP: REP,
    LESS_EQUAL_REP: REP,
    LESS_ASCII_REP: REP,
    LESS_EQUAL_ASCII_REP: REP,
  },
  {
    DOUBLE_VERTICAL_BAR_REP: REP,
    NON_DOUBLE_VERTICAL_BAR_REP: REP,
  },
  {
    SAME_Q_REP: REP,
    UNSAME_Q_REP: REP,
  },
  {
    ELEMENT_REP: REP,
    NOT_ELEMENT_REP: REP,
    SUBSET_REP: REP,
    SUPERSET_REP: REP,
  },
  {
    NOT: prec,
    NEGATE: prec,
  },
  {
    AND_REP: REP,
    AND_WEDGE_REP: REP,
    NAND_WEDGE_REP: REP,
  },
  {
    XOR_REP: REP,
    XNOR_REP: REP,
  },
  {
    OR_REP: REP,
    OR_VEE_REP: REP,
    NOR_VEE_REP: REP,
  },
  {
    EQUIVALENT_REP: REP,
  },
  {
    IMPLIES: prec.right,
    ROUND_IMPLIES: prec.right,
  },
  {
    RIGHT_TEE: prec.right,
    DOUBLE_RIGHT_TEE: prec.right,
    LEFT_TEE: prec.left,
    DOUBLE_LEFT_TEE: prec.left,
    UP_TEE: prec.left,
    DOWN_TEE: prec.left,
  },
  {
    SUCH_THAT: prec.right,
  },
  {
    REPEATED: prec,
    REPEATED_NULL: prec,
  },
  {
    ALTERNATIVES_REP: REP,
  },
  {
    PATTERN: prec,
    OPTIONAL_PATTERN: prec,
    OPTIONAL: prec,
  },
  {
    STRING_EXPRESSION_REP: REP,
  },
  {
    CONDITION: prec.left,
  },
  {
    TWO_WAY_RULE: prec.right,
  },
  {
    RULE_ASCII: prec.right,
    RULE: prec.right,
    RULE_DELAYED_ASCII: prec.right,
    RULE_DELAYED: prec.right,
  },
  {
    REPLACE_ALL: prec.left,
    REPLACE_REPEATED: prec.left,
  },
  {
    ADD_TO: prec.right,
    SUBTRACT_FROM: prec.right,
    TIMES_BY: prec.right,
    DIVIDE_BY: prec.right,
  },
  {
    FUNCTION_ANONYMOUS: prec,
  },
  {
    COLON_REP: REP,
  },
  {
    APPLY_TO: prec.right,
  },
  {
    BACK_APPLY: prec.left,
  },
  {
    VERTICAL_SEPARATOR_REP: REP,
  },
  {
    THEREFORE: prec.right,
    BECAUSE: prec.left,
  },
  {
    SET: prec.right,
    SET_DELAYED: prec.right,
    UP_SET: prec.right,
    UP_SET_DELAYED: prec.right,
    TAG_SET: prec.right,
    TAG_SET_DELAYED: prec.right,
    UNSET: prec.right,
    TAG_UNSET: prec.right,
    FUNCTION_ARROW: prec.right,
  },
  {
    PUT: prec,
    PUT_APPEND: prec,
  },

  //
  {
    COMPOUND_EXPRESSION_REP: prec,
  },
];
/**
 * @type Tmp2<
 *   keyof typeof PREC_ORIG[number]
 * >
 */
const PREC = (() => {
  /** @type any */
  const tmp = Object.fromEntries(
    PREC_ORIG.flatMap((e, i) =>
      Object.entries(e).flatMap(([key, grouping]) =>
        key.endsWith("_REP")
          ? [
              [
                key,
                (/** @type RuleOrLiteral */ rule, /** @type RuleOrLiteral */ sepRule) =>
                  prec.left(-i * 2, seq0(rule, repeat1(prec(-i * 2 + 1, seq0(sepRule, rule))))),
              ],
              [key.slice(0, -"_REP".length), (/** @type any */ rule) => prec(-i * 2, rule)],
            ]
          : [[key, (/** @type any */ rule) => grouping(-i * 2, rule)]],
      ),
    ),
  );
  return tmp;
})();

/**
 * @type Tmp5
 */
const tmp5 = (() => {
  /** @type any */
  const tmp = (/** @type any */ rules) => {
    const keys = Object.keys(rules);
    const op_names = keys.filter(
      (e) => !e.endsWith("_operator") && !e.endsWith("_operator_left") && !e.endsWith("_operator_right"),
    );
    return {
      ...Object.fromEntries(
        Object.entries(rules).flatMap(([name, builder]) => {
          const replacer =
            (
              /** @type any */ symb,
              /** @type any */ expr,
              /** @type any */ expr1,
              /** @type any */ expr1_unary,
              /** @type any */ nullable_expr,
            ) =>
            (/** @type any */ e) => {
              if (e === SYMB_KEEP) {
                return symb;
              }
              if (e === EXPR_KEEP) {
                return expr;
              }
              if (e === EXPR1_KEEP) {
                return expr1;
              }
              if (e === EXPR1_UNARY_KEEP) {
                return expr1_unary;
              }
              if (e === NULLABLE_EXPR_KEEP) {
                return nullable_expr;
              }
              return e;
            };
          return [
            [
              name,
              (/** @type any */ $) => {
                globalReplacer = replacer(
                  //
                  $._expr1_symbol,
                  $._expr,
                  $._expr1,
                  $._expr1_unary,
                  choice($._expr),
                );
                const r = builder($);
                globalReplacer = defaultReplacer;
                return r;
              },
            ],
            [
              `${name}_top`,
              (/** @type any */ $) => {
                globalReplacer = replacer(
                  //
                  $._expr1_symbol_top,
                  $._expr_top,
                  $._expr1_top,
                  $._expr1_unary_top,
                  $._expr_top,
                );
                const r = builder($);
                globalReplacer = defaultReplacer;
                return r;
              },
            ],
          ];
        }),
      ),
      _expr_op_top: (/** @type any */ $) => choice(...op_names.map((e) => $[`${e}_top`])),
      _expr_op: (/** @type any */ $) => prec(35, choice(...op_names.map((e) => $[e]))),
    };
  };
  return tmp;
})();

module.exports = grammar0({
  name: "wolfram",
  extras: ($) => [/[ \t]/, $.comment],
  externals: (/** @type any */ $) => [$.comment],
  conflicts: ($) => [
    [$.underscript, $.underoverscript, $.overunderscript],
    [$.underscript_top, $.underoverscript_top, $.overunderscript_top],
    [$.overscript, $.underscript, $.underoverscript],
    [$.overscript_top, $.underscript_top, $.underoverscript_top],
    [$.overscript_operator, $.overunderscript_operator_left],
    [$.underscript_operator, $.underoverscript_operator_left],
    [$.subscript_operator, $.subscript_power_operator_left],
    [$.tilde_operator_left, $.tilde_operator_right],
    [$.power, $.power_subscript],
    [$.power_top, $.power_subscript_top],
    [$.sqrt, $.any_root],
    [$.sqrt_top, $.any_root_top],
    [$._expr1_unary],
    [$._expr1],
    [$._expr1_unary_special, $._expr1],
    [$._expr1_unary_special_top, $._expr1_top],
    [$._expr1_unary, $._expr1_unary_special],
    [$._expr1_unary_top, $._expr1_unary_special_top],
    [$._expr1_unary, $._expr1],
    [$._expr1_unary_top, $._expr1_top],
    [$._expr1_unary, $._expr_op],
    [$._expr1_unary_top, $._expr_op_top],
    [$._expr1_unary_special_top, $._expr_op_top],
    [$._expr1_unary, $._expr1_unary_special, $._expr_op],
    [$._expr1_unary_special, $._expr_op],
    [$._expr1_unary_special_top],
    [$._expr1_unary, $._expr1_unary_special, $._expr1],
    [$._expr1_unary_special],
    [$.compound_expression],
    [$._expr1, $.compound_expression],
    [$._expr1_unary, $._expr1_unary_special, $._expr1, $.compound_expression],
    [$.tag_set_operator_left, $.tag_set_delayed_operator_left, $.tag_unset_operator_left],
    [$._expr1_symbol],
    [$.set_operator, $.tag_set_operator_right],
    [$.set_delayed_operator, $.tag_set_delayed_operator_right],
    [$.unset_operator, $.tag_unset_operator_right],
  ],

  rules: {
    comment: EXTERNAL,

    source_file: ($) =>
      seq(
        // drop starting empty lines
        repeat(/\n+/),
        optional(seq(repeat(seq($._expr_top, repeat1(/\n+/))), $._expr_top, repeat(/\n+/))),
      ),
    _parenthesized_expr: ($) => seq($.parenthesis_begin, $._expr, $.parenthesis_end),
    parenthesis_begin: (_$) => "(",
    parenthesis_end: (_$) => ")",
    _expr_finished: ($) =>
      choice(
        $._parenthesized_expr,
        $._symbol,
        $.blank,
        $.blank_sequence,
        $.blank_null_sequence,
        $.string,
        $.number,

        // op
        $.list,
        $.association,
        $.apply,
        $.part,
      ),
    _expr_top: ($) => prec.left(30, choice(seq($._expr1_top, repeat($._expr1_unary_special_top)))),
    _expr: ($) => prec(20, choice(prec.left(seq($._expr1_unary_special, repeat1($._expr1_unary_special))), $._expr1)),
    _expr1_symbol_top: ($) => prec(10, choice($._symbol)),
    _expr1_symbol: ($) => prec(10, seq(repeat(/\n+/), choice($._symbol))),
    _expr1_unary_top: ($) =>
      prec(
        20,
        choice(
          $._expr_finished,
          //
          $.positive_top,
          $.negative_top,
          $.negative_positive_top,
          $.increment_top,
          $.decrement_top,
          $.pre_increment_top,
          $.pre_decrement_top,
          $.del_top,
        ),
      ),
    _expr1_unary: ($) =>
      prec(
        30,
        seq(
          repeat(/\n+/),
          choice(
            $._expr_finished,
            //
            $.positive,
            $.negative,
            $.negative_positive,
            $.increment,
            $.decrement,
            $.pre_increment,
            $.pre_decrement,
            $.del,
          ),
          repeat(/\n+/),
        ),
      ),
    _expr1_unary_special_top: ($) =>
      seq(
        choice(
          $._expr_finished,
          //
          $.increment_top,
          $.decrement_top,
          $.pre_increment_top,
          $.pre_decrement_top,
          $.del_top,
        ),
      ),
    _expr1_unary_special: ($) =>
      prec.left(
        seq(
          repeat(/\n+/),
          choice(
            $._expr_finished,
            //
            $.increment,
            $.decrement,
            $.pre_increment,
            $.pre_decrement,
            $.del,
          ),
          repeat(/\n+/),
        ),
      ),
    _expr1_top: ($) => choice($._expr_finished, $._expr_op_top),
    _expr1: ($) =>
      prec.left(
        32,
        seq(
          repeat(/\n+/),
          choice(
            //
            prec.left(40, $._expr_finished),
            $._expr_op,
          ),
          repeat(/\n+/),
        ),
      ),
    _expr_repeat: ($) => seq($._expr, repeat(seq(",", $._expr))),

    // <OPERATORS>

    ...tmp5({
      // <UNARY>

      increment: ($) => PREC.INCREMENT(seq0(EXPR1_UNARY_KEEP, $.increment_operator)),
      increment_operator: (_$) => "++",

      decrement: ($) => PREC.DECREMENT(seq0(EXPR1_UNARY_KEEP, $.decrement_operator)),
      decrement_operator: (_$) => "--",

      pre_increment: ($) => PREC.PRE_INCREMENT(seq0($.pre_increment_operator, EXPR1_UNARY_KEEP)),
      pre_increment_operator: (_$) => "++",

      pre_decrement: ($) => PREC.PRE_DECREMENT(seq0($.pre_decrement_operator, EXPR1_UNARY_KEEP)),
      pre_decrement_operator: (_$) => "--",

      positive: ($) => PREC.POSITIVE(seq0($.positive_operator, EXPR1_UNARY_KEEP)),
      positive_operator: (_$) => "+",

      negative: ($) => PREC.NEGATIVE(seq0($.negative_operator, EXPR1_UNARY_KEEP)),
      negative_operator: (_$) => "-",

      negative_positive: ($) => PREC.NEGATIVE_POSITIVE(seq0($.negative_positive_operator, EXPR1_UNARY_KEEP)),
      negative_positive_operator: (_$) => "±",

      del: ($) => PREC.DEL(seq0($.del_operator, EXPR1_UNARY_KEEP)),
      del_operator: (_$) => "∇",

      // </UNARY>

      subscript: ($) => PREC.SUBSCRIPT(seq0(EXPR_KEEP, $.subscript_operator, EXPR_KEEP)),
      subscript_operator: (_$) => "\\_",

      subscript_power: ($) =>
        PREC.SUBSCRIPT_POWER(
          seq0(EXPR_KEEP, $.subscript_power_operator_left, EXPR_KEEP, $.subscript_power_operator_right, EXPR_KEEP),
        ),
      subscript_power_operator_left: (_$) => "\\_",
      subscript_power_operator_right: (_$) => "\\%",

      compound_expression: ($) =>
        prec.left(
          40000,
          seq0(
            NULLABLE_EXPR_KEEP,
            prec(8000, repeat1(prec(450, seq0($.compound_expression_operator, NULLABLE_EXPR_KEEP)))),
          ),
        ),
      compound_expression_operator: (_$) => ";",

      set: ($) => PREC.SET(seq0(EXPR_KEEP, $.set_operator, EXPR_KEEP)),
      set_operator: (_$) => "=",

      set_delayed: ($) => PREC.SET_DELAYED(seq0(EXPR_KEEP, $.set_delayed_operator, EXPR_KEEP)),
      set_delayed_operator: (_$) => ":=",

      up_set: ($) => PREC.UP_SET(seq0(EXPR_KEEP, $.up_set_operator, EXPR_KEEP)),
      up_set_operator: (_$) => "^=",

      up_set_delayed: ($) => PREC.UP_SET_DELAYED(seq0(EXPR_KEEP, $.up_set_delayed_operator, EXPR_KEEP)),
      up_set_delayed_operator: (_$) => "^:=",

      // unary but...
      unset: ($) => PREC.UNSET(seq0(EXPR_KEEP, $.unset_operator)),
      unset_operator: (_$) => "=.",

      tag_set: ($) =>
        PREC.TAG_SET(seq0(SYMB_KEEP, $.tag_set_operator_left, EXPR_KEEP, $.tag_set_operator_right, EXPR_KEEP)),
      tag_set_operator_left: (_$) => "/:",
      tag_set_operator_right: (_$) => "=",

      tag_set_delayed: ($) =>
        PREC.TAG_SET_DELAYED(
          seq0(SYMB_KEEP, $.tag_set_delayed_operator_left, EXPR_KEEP, $.tag_set_delayed_operator_right, EXPR_KEEP),
        ),
      tag_set_delayed_operator_left: (_$) => "/:",
      tag_set_delayed_operator_right: (_$) => ":=",

      tag_unset: ($) =>
        PREC.TAG_SET_DELAYED(seq0(SYMB_KEEP, $.tag_unset_operator_left, EXPR_KEEP, $.tag_unset_operator_right)),
      tag_unset_operator_left: (_$) => "/:",
      tag_unset_operator_right: (_$) => "=.",

      pattern_test: ($) => PREC.PATTERN_TEST(seq0(EXPR_KEEP, $.pattern_test_operator, EXPR_KEEP)),
      pattern_test_operator: (_$) => "?",

      equal_ascii: ($) => PREC.EQUAL_REP(EXPR_KEEP, $.equal_ascii_operator),
      equal_ascii_operator: (_$) => "==",

      unequal_ascii: ($) => PREC.UNEQUAL_ASCII_REP(EXPR_KEEP, $.unequal_ascii_operator),
      unequal_ascii_operator: (_$) => "!=",

      greater_ascii: ($) => PREC.GREATER_ASCII_REP(EXPR_KEEP, $.greater_ascii_operator),
      greater_ascii_operator: (_$) => ">",

      greater_equal_ascii: ($) => PREC.GREATER_EQUAL_ASCII_REP(EXPR_KEEP, $.greater_equal_ascii_operator),
      greater_equal_ascii_operator: (_$) => ">=",

      less_ascii: ($) => PREC.LESS_ASCII_REP(EXPR_KEEP, $.less_ascii_operator),
      less_ascii_operator: (_$) => "<",

      less_equal_ascii: ($) => PREC.LESS_EQUAL_ASCII_REP(EXPR_KEEP, $.less_equal_ascii_operator),
      less_equal_ascii_operator: (_$) => "<=",

      rule_ascii: ($) => PREC.RULE_ASCII(seq0(EXPR_KEEP, $.rule_ascii_operator, EXPR_KEEP)),
      rule_ascii_operator: (_$) => "->",

      rule_delayed_ascii: ($) => PREC.RULE_DELAYED_ASCII(seq0(EXPR_KEEP, $.rule_delayed_ascii_operator, EXPR_KEEP)),
      rule_delayed_ascii_operator: (_$) => ":>",

      message_name: ($) => PREC.MESSAGE_NAME_REP(EXPR_KEEP, $.message_name_operator),
      message_name_operator: (_$) => "::",

      apply_at: ($) => PREC.APPLY_AT(seq0(EXPR_KEEP, $.apply_at_operator, EXPR_KEEP)),
      apply_at_operator: (_$) => "@",

      tilde: ($) => PREC.TILDE(seq0(EXPR_KEEP, $.tilde_operator_left, EXPR_KEEP, $.tilde_operator_right, EXPR_KEEP)),
      tilde_operator_left: (_$) => "~",
      tilde_operator_right: (_$) => "~",

      composition: ($) => PREC.COMPOSITION_REP(EXPR_KEEP, $.composition_operator),
      composition_operator: (_$) => "@*",

      right_composition: ($) => PREC.RIGHT_COMPOSITION_REP(EXPR_KEEP, $.right_composition_operator),
      right_composition_operator: (_$) => "/*",

      get: ($) => PREC.GET(seq0($.get_operator, $.string)),
      get_operator: (_$) => "<<",

      overscript: ($) => PREC.OVERSCRIPT(seq0(EXPR_KEEP, $.overscript_operator, EXPR_KEEP)),
      overscript_operator: (_$) => "\\&",

      underscript: ($) => PREC.UNDERSCRIPT(seq0(EXPR_KEEP, $.underscript_operator, EXPR_KEEP)),
      underscript_operator: (_$) => "\\+",

      underoverscript: ($) =>
        PREC.UNDEROVERSCRIPT(
          seq0(EXPR_KEEP, $.underoverscript_operator_left, EXPR_KEEP, $.underoverscript_operator_right, EXPR_KEEP),
        ),
      underoverscript_operator_left: (_$) => "\\+",
      underoverscript_operator_right: (_$) => "\\%",

      overunderscript: ($) =>
        PREC.OVERUNDERSCRIPT(
          seq0(EXPR_KEEP, $.overunderscript_operator_left, EXPR_KEEP, $.overunderscript_operator_right, EXPR_KEEP),
        ),
      overunderscript_operator_left: (_$) => "\\&",
      overunderscript_operator_right: (_$) => "\\%",

      // https://reference.wolfram.com/language/ref/Function.html
      function_anonymous: ($) => PREC.FUNCTION_ANONYMOUS(seq0(EXPR_KEEP, $.function_anonymous_operator)),
      function_anonymous_operator: (_$) => "&",

      function_arrow: ($) => PREC.FUNCTION_ARROW(seq0(EXPR_KEEP, $.function_arrow_operator, EXPR_KEEP)),
      function_arrow_operator: (_$) => "|->",

      small_circle: ($) => PREC.SMALL_CIRCLE_REP(EXPR_KEEP, $.small_circle_operator),
      small_circle_operator: (_$) => "∘",

      circle_dot: ($) => PREC.CIRCLE_DOT_REP(EXPR_KEEP, $.circle_dot_operator),
      circle_dot_operator: (_$) => "⊙",

      non_commutative_multiply: ($) =>
        PREC.NON_COMMUTATIVE_MULTIPLY_REP(EXPR_KEEP, $.non_commutative_multiply_operator),
      non_commutative_multiply_operator: (_$) => "**",

      dot: ($) => PREC.DOT_REP(EXPR_KEEP, $.dot_operator),
      dot_operator: (_$) => ".",

      backslash: ($) => PREC.BACKSLASH_REP(EXPR_KEEP, $.backslash_operator),
      backslash_operator: (_$) => "\\",

      diamond: ($) => PREC.DIAMOND_REP(EXPR_KEEP, $.diamond_operator),
      diamond_operator: (_$) => "⋄",

      wedge: ($) => PREC.WEDGE_REP(EXPR_KEEP, $.wedge_operator),
      wedge_operator: (_$) => "⋀",

      vee: ($) => PREC.WEDGE_REP(EXPR_KEEP, $.vee_operator),
      vee_operator: (_$) => "⋁",

      circle_times: ($) => PREC.CIRCLE_TIMES_REP(EXPR_KEEP, $.circle_times_operator),
      circle_times_operator: (_$) => "⊗",

      center_dot: ($) => PREC.CENTER_DOT_REP(EXPR_KEEP, $.center_dot_operator),
      center_dot_operator: (_$) => "·",

      times_ast: ($) => PREC.TIMES_AST_REP(EXPR_KEEP, $.times_ast_operator),
      times_ast_operator: (_$) => "*",

      times_cross: ($) => PREC.TIMES_AST_REP(EXPR_KEEP, $.times_cross_operator),
      times_cross_operator: (_$) => "×",

      star: ($) => PREC.STAR_REP(EXPR_KEEP, $.star_operator),
      star_operator: (_$) => "⋆",

      vertical_tilde: ($) => PREC.VERTICAL_TILDE_REP(EXPR_KEEP, $.vertical_tilde_operator),
      vertical_tilde_operator: (_$) => "≀",

      coproduct: ($) => PREC.COPRODUCT_REP(EXPR_KEEP, $.coproduct_operator),
      coproduct_operator: (_$) => "∐",

      circle_plus: ($) => PREC.CIRCLE_PLUS_REP(EXPR_KEEP, $.circle_plus_operator),
      circle_plus_operator: (_$) => "⊕",

      circle_minus: ($) => PREC.CIRCLE_MINUS(seq0(EXPR_KEEP, $.circle_minus_operator, EXPR_KEEP)),
      circle_minus_operator: (_$) => "⊖",

      plus: ($) => PREC.PLUS_REP(EXPR_KEEP, $.plus_operator),
      plus_operator: (_$) => "+",

      minus: ($) => PREC.MINUS_REP(EXPR_KEEP, $.minus_operator),
      minus_operator: (_$) => PREC.MINUS("-"),

      plus_minus: ($) => PREC.PLUS_MINUS(seq0(EXPR_KEEP, $.plus_minus_operator, EXPR_KEEP)),
      plus_minus_operator: (_$) => PREC.PLUS_MINUS("±"),

      per: ($) => PREC.PER(seq0(EXPR_KEEP, $.per_operator, EXPR_KEEP)),
      per_operator: (_$) => "/",

      divide: ($) => PREC.DIVIDE(seq0(EXPR_KEEP, $.divide_operator, EXPR_KEEP)),
      divide_operator: (_$) => "÷",

      divide_box: ($) => PREC.DIVIDE_BOX(seq0(EXPR_KEEP, $.divide_box_operator, EXPR_KEEP)),
      divide_box_operator: (_$) => "\\/",

      // https://reference.wolfram.com/language/ref/Map.html
      map: ($) => PREC.MAP(seq0(EXPR_KEEP, $.map_operator, EXPR_KEEP)),
      map_operator: (_$) => "/@",

      map_all: ($) => PREC.MAP_ALL(seq0(EXPR_KEEP, $.map_all_operator, EXPR_KEEP)),
      map_all_operator: (_$) => "//@",

      factorial: ($) => PREC.FACTORIAL(seq0(EXPR_KEEP, $.factorial_operator)),
      factorial_operator: (_$) => "!",

      factorial2: ($) => PREC.FACTORIAL2(seq0(EXPR_KEEP, $.factorial2_operator)),
      factorial2_operator: (_$) => "!!",

      derivative: ($) => PREC.DERIVATIVE(seq0(EXPR_KEEP, $.derivative_operator)),
      derivative_operator: (_$) => prec.right(repeat1("'")),

      sqrt: ($) => PREC.SQRT(seq0(EXPR_KEEP, $.sqrt_operator)),
      sqrt_operator: (_$) => PREC.SQRT("\\@"),

      any_root: ($) => PREC.ANY_ROOT(seq0($.any_root_operator_left, EXPR_KEEP, $.any_root_operator_right, EXPR_KEEP)),
      any_root_operator_left: (_$) => PREC.ANY_ROOT("\\@"),
      any_root_operator_right: (_$) => "\\%",

      power: ($) => PREC.POWER(seq0(EXPR_KEEP, $.power_operator, EXPR_KEEP)),
      power_operator: (_$) => "^",

      power_subscript: ($) =>
        PREC.POWER_SUBSCRIPT(
          seq0(EXPR_KEEP, $.power_subscript_operator_left, EXPR_KEEP, $.power_subscript_operator_right, EXPR_KEEP),
        ),
      power_subscript_operator_left: (_$) => "\\^",
      power_subscript_operator_right: (_$) => "\\%",

      // https://reference.wolfram.com/language/ref/Apply.html
      apply_atat: ($) => PREC.APPLY_ATAT(seq0(EXPR_KEEP, $.apply_atat_operator, EXPR_KEEP)),
      apply_atat_operator: (_$) => "@@",

      // https://reference.wolfram.com/language/ref/Apply.html
      apply_atatat: ($) => PREC.APPLY_ATATAT(seq0(EXPR_KEEP, $.apply_atatat_operator, EXPR_KEEP)),
      apply_atatat_operator: (_$) => "@@@",

      alternatives: ($) => PREC.ALTERNATIVES_REP(EXPR_KEEP, $.alternatives_operator),
      alternatives_operator: (_$) => "|",

      string_join: ($) => PREC.STRING_JOIN_REP(EXPR_KEEP, $.string_join_operator),
      string_join_operator: (_$) => "<>",
    }),

    list: ($) => seq0($.list_bracket_begin, choice($._expr_repeat, repeat(/\n+/)), $.list_bracket_end),
    list_bracket_begin: (_$) => "{",
    list_bracket_end: (_$) => "}",

    apply: ($) =>
      PREC.APPLY(
        seq0($._expr_finished, $.apply_bracket_begin, choice($._expr_repeat, repeat(/\n+/)), $.apply_bracket_end),
      ),
    apply_bracket_begin: (_$) => "[",
    apply_bracket_end: (_$) => "]",

    association: ($) =>
      seq0($.association_bracket_begin, choice($._expr_repeat, repeat(/\n+/)), $.association_bracket_end),
    association_bracket_begin: (_$) => "<|",
    association_bracket_end: (_$) => "|>",

    part: ($) =>
      PREC.PART(
        seq0($._expr_finished, $.part_bracket_begin, choice($._expr_repeat, repeat(/\n+/)), $.part_bracket_end),
      ),
    part_bracket_begin: (_$) => "[[",
    part_bracket_end: (_$) => "]]",

    // </OPERATORS>

    _symbol: ($) => choice($.user_symbol, $.builtin_symbol),
    user_symbol: ($) => $._lower_identifier,
    builtin_symbol: ($) => $._upper_identifier,

    blank: (_$) => /_(?:[a-zA-Z][a-zA-Z0-9]*)?/,
    blank_sequence: (_$) => /__(?:[a-zA-Z][a-zA-Z0-9]*)?/,
    blank_null_sequence: (_$) => /___(?:[a-zA-Z][a-zA-Z0-9]*)?/,

    string: ($) =>
      seq($.string_begin, repeat(choice(/[^"\\\n]/, $.string_char_escape, $.string_char_name_escape)), $.string_end),
    string_char_escape: (_$) => choice('\\"', "\\\\", "\\n", "\\r", "\\t", "\\t", "\\v"),
    string_char_name_escape: ($) => seq("\\[", $.string_char_name, "]"),
    string_char_name: ($) => $._upper_identifier,
    string_begin: (_$) => '"',
    string_end: (_$) => '"',

    // https://wltools.github.io/LanguageSpec/Specification/Syntax/Number-representations/
    // Special character input forms are not supported.
    number: (_$) =>
      /((([2-9]|[1-2]\d|[3][0-5])\^\^(\w*\.\w+|\w+\.\w*|\w+))|(\d*\.\d+|\d+\.\d*|\d+))((``(\+|-)?(\d*\.\d+|\d+\.\d*|\d+))|(`((\+|-)?(\d*\.\d+|\d+\.\d*|\d+))?))?(\*\^(\+|-)?\d+)?/,

    _lower_identifier: (_$) => /[a-z][a-zA-Z0-9]*/,
    _upper_identifier: (_$) => /[$A-Z][a-zA-Z0-9]*/,
  },
});
