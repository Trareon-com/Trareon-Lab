rule Text_And_Hex {
  meta:
    severity = "HIGH"
    family = "golden"
  strings:
    $text = "alpha marker" nocase
    $magic = { DE AD BE EF }
  condition:
    $text and $magic
}

rule Text_Or_Text {
  meta:
    severity = "LOW"
  strings:
    $one = "first choice"
    $two = "second choice"
  condition:
    $one or $two
}
