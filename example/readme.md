# Rupert Example

Acts as a proof-of-concept for rupert, parsing something like this...

```
3 + 9 || 4 - 2 && 32 > 31 - 43 * 2 + '{\"Hello\"}{34}'
```

- using the correct order of operations
- binary expressions are parsed as the logical form of the tree without

  - left or right infinite recursion
  - having to correct the tree after making the standard [left recursion fixes](https://youtu.be/H7iGUr2W5N8?si=IPL0cPhihDgfZSVo)
