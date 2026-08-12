module.exports = {
  yamluml: ($) => seq("@startyaml", optional(field("yaml", $.commandyaml)), "@endyaml"),
  commandyaml: ($) => repeat1($._commandyaml),
  _commandyaml: (_$) => /[^\n]+/g,
};
