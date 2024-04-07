# Rupert

A simple parsing library. For a usage example, see the `example` folder.

## The Problem

Specifically, this is an investigation into how to write a recursive-descent parser that does not suffer from infinite left-recursion issues.

For example, this sort of grammar can tend to start an infinite loop:

```
NumberLiteral: /[0-9]+/
MultiplicativeExpr: Expr ("*"|"/") Expr
AdditiveExpr: Expr ("+"|"-") Expr
Expr: NumberLiteral | MultiplicativeExpr | AdditiveExpr
```

Of course, you could refactor the rules so that the left recursion is eliminated, but that means that the parse tree doesn't have the simplicity of the grammar above.

And of course, you could just have a post-process step to fix the tree, but who has the time for that?

## The Solution

To solve this, Rupert uses a system of wrapping.

Rules that could be left recursive are wrapped, that is parsed from the bottom up, instead of from the top down. This doesn't change the fact that most of the grammar is parsed from the top down though.

```
2 + 2
||| ^ ---- try `Expr`, found `NumberLiteral`. `AdditiveExpression` is complete
||^ ----- oh yes, we can. This is an `AdditiveExpression`
|^ ----- we see if we can wrap the parsed `NumberLiteral`
^ ----- try `Expr`, found `NumberLiteral`, all good
```
