---
layout: "page"
title: Test
---
<h1>{{ "Hello World!" | downcase }}</h1>

{% assign k = site.data.menu | keys %}
{% for key in k %}
<h2>{{ key }}</h2>
{% endfor %}
