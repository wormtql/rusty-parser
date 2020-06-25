# Rust Parser

## 功能
进行LR、LL分析  
可以将语法描述文件缓存为LR表

## 语法描述文件的语法
```
ORIGIN S
    A B
    B A

A
    @a A
    EMPTY

B
    @b B
    EMPTY
```
以上等价于
```
S -> AB|BA
A -> aA|ε
B -> bB|ε
```

## 使用实例
```bash
parser --cache grammar_test/c2.txt -o grammar_test/c2.cache --lr1
parser --lr-analysis --token=grammar_test/token.txt --cached-lr-table=grammar_test/c2.cache -o grammar_test/cst.json
```