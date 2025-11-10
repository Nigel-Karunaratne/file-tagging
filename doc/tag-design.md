# Tag Design

Tags use a key-value system, enabling one "tag" to be used in multiple contexts

Example

```
Texture/Verso
Model/Verso
Weapon/Verso

Due/NextWeek
Due/Today
Due/Unknown
```

Most Tags do not need the "value" attribute

```
Personal
Work
School
```

Tags can be queried by Key tags only, by both Key and Value tags, or by Value tags only.

Using the first example, queries are possible for:
- All Textures
- All files for "Verso"
- All Textures for "Verso"
