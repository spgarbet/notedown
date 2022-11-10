---
layout: "page"
title: Test
---
<h1>{{ "Hello World!" | downcase }}</h1>

{% assign k = site.data.menu | keys %}
<ul>
{% for key in k %}
<li><h2>{{ key }}</h2>
  <ul>
  {% assign tops = site.data.menu[key] | keys %}
  {% for t in tops %}
  <li><h3>{{ t }}</h3>
    <ul>
    {% for e in site.data.menu[key][t] %}
    <li><a href="file://{{e.href}}">{{e.title}}</a></li>
    {% endfor %}
    </ul>
  </li>
  {% endfor %}
  </ul>
</li>
{% endfor %}
</ul>
