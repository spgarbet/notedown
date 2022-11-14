---
layout: "page"
title: Notedown
---
<h1>Notedown</h1>
<div class="notedown">
{% assign k = site.data.menu | keys %}
{% for key in k %}
<div class="topic">
<h2>{{ key }}</h2>
  <ul>
  {% assign tops = site.data.menu[key] | keys %}
  {% for t in tops %}
  <li onclick="$('ul#{{key}}{{t}}').toggle()"><h3>{{ t }} <small>({{site.data.menu[key][t] | size}})</small></h3>
    <ul id="{{key}}{{t}}">
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
