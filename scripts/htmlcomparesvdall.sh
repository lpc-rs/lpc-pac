#!/usr/bin/env bash
set -euxo pipefail



mkdir -p html/lpc546xx

mkdir -p html/lpc546xx/lpc54605
python3 scripts/htmlcomparesvd.py html/lpc546xx/lpc54605 svd/lpc54605.svd.patched

cat > html/comparisons.html <<EOF
<!DOCTYPE html>
<head>
<meta charset="utf-8">
<meta name=viewport content="width=device-width, initial-scale=1">
<title>lpc-pac Peripheral Comparisons</title>
<link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css" integrity="sha384-BVYiiSIFeK1dGmJRAkycuHAHRg32OmUcww7on3RYdg4Va+PmSTsz/K68vbdEjh4u" crossorigin="anonymous">
<link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap-theme.min.css" integrity="sha384-rHyoN1iRsVXV4nD0JutlnGaslCJuC7uwjduW9SVrLvRYooPp2bWYgmgJQIXwl/Sp" crossorigin="anonymous">
</head>
<body>
<nav class="navbar navbar-inverse">
  <div class="container">
    <div class="navbar-header">
      <a class="navbar-brand" href="index.html">lpc-pac Device Coverage</a>
    </div>
    <div class="navbar-collapse collapse">
      <ul class="nav navbar-nav">
        <li class="active"><a href="#">Peripheral Comparisons</a></li>
      </ul>
    </div>
  </div>
</nav>
<h1>Device families</h1>
<ul>
  <li><a href="lpc546xx/index.html">lpc546xx</a></li>
</ul>
</body>
</html>
EOF