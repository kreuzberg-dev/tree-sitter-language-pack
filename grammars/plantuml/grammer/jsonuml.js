module.exports = {
  jsonuml: ($) => seq("@startjson", optional(field("json", $.commandjson)), "@endjson"),
  commandjson: ($) => repeat1($._commandjson),
  _commandjson: (_$) => /[^\n]+/g,
};
