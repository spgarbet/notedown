---
layout: "page"
title: Test
---
<html>
<head>
<title>Notedown</title>
<style type="text/css">
.notedown
{
    display: flex;
    justify-content: space-between;
    margin: auto;
}

.topic
{
    flex: 1;
}
</style>
</head>
<body>

<h1>Notedown</h1>

<div class="notedown">

{% assign k = site.data.menu | keys %}
{% for key in k %}
<div class="topic">
<h2>{{ key }}</h2>
  <ul>
  {% assign tops = site.data.menu[key] | keys %}
  {% for t in tops %}
  <li><h3>{{ t }}</h3>
    <ul>
    {% for e in site.data.menu[key][t] %}
    <li><small>{{e.date}}</small> <a href="file://{{e.href}}">{{e.title}}</a></li>
    {% endfor %}
    </ul>
  </li>
  {% endfor %}
  </ul>
</div>
{% endfor %}

</div>

</body>
</html>
