# Render a jscpd JSON report as a Markdown summary, used by `just ci-cpd` for the PR comment.
# Usage: jq -r --arg base <git-ref> -f jscpd-summary.jq target/jscpd/jscpd-report.json
# Set CPD_BLOB_URL (e.g. https://github.com/<owner>/<repo>/blob/<sha>) to link each clone to its source.

def plural($n; $word): "\($n) \($word)\(if $n == 1 then "" else "s" end)";
def loc: "\(.name):\(.start)-\(.end)" as $text
  | if $ENV.CPD_BLOB_URL then "[`\($text)`](\($ENV.CPD_BLOB_URL)/\(.name)#L\(.start)-L\(.end))" else "`\($text)`" end;
def max_rows: 50;

.statistics.total as $t
| (.duplicates | sort_by(if .isNew then 0 else 1 end, -.lines)) as $clones
| "### Copy/paste detection\n",
  if $t.clones == 0 then
    "✅ No duplicated code found in \(plural($t.sources; "file")) (\($t.lines) lines)."
  else
    (if $t.newClones > 0 then "⚠️" else "ℹ️" end)
    + " Found **\(plural($t.clones; "clone"))** (**\($t.newClones) new** compared to `\($base)`)"
    + " with \($t.duplicatedLines) duplicated lines — \($t.percentage * 100 | round / 100)% of \($t.lines) lines in \(plural($t.sources; "file")).\n",
    "<details\(if $t.newClones > 0 then " open" else "" end)><summary>Clones</summary>\n",
    "| New | Lines | Tokens | Location | Duplicated in |",
    "|:---:|------:|-------:|----------|---------------|",
    ($clones[:max_rows][]
      | "| \(if .isNew then "🆕" else "" end) | \(.lines) | \(.tokens) | \(.firstFile | loc) | \(.secondFile | loc) |"),
    (if ($clones | length) > max_rows then "\n…and \(($clones | length) - max_rows) more." else empty end),
    "\n</details>"
  end
