# Conditional Forwarding

Conditional forwarding allows data to be processed and then forwarded based on the `Tags` returned in the result. This document outlines the different logical operations that can be performed on tags.

* **and** - The message is forwarded if all the tags specified in the Message are present in the edge conditions.
* **or** - The message is forwarded if any of the tags specified in the Message are present in the edge conditions.
* **not** - The message is forwarded if the tags specified in the Message are not present in the edge conditions.

Consider an example where a UDF is used to process numbers and forward the result to different vertices based on whether the number is even or odd. In this case, you can set the `tag` to `even-tag` or `odd-tag` in each of the returned messages, and define the edges as follows:

```yaml
edges:
  - from: p1
    to: even-vertex
    conditions:
      tags:
        operator: or # Optional, defaults to "or".
        values:
          - even-tag
  - from: p1
    to: odd-vertex
    conditions:
      tags:
        operator: not
        values:
          - odd-tag
  - from: p1
    to: all
    conditions:
      tags:
        operator: and
        values:
          - odd-tag
          - even-tag