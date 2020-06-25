# Exam Cheater
### 为确保每个人得出的答案不同，需要设定随机种子（环境变量）
windows:
```bash
set SEED=10086
```

linux/macos（大概）:
```bash
export SEED=10086
```
只要种子一样，所有的结果都是相同的
### 语法文件的格式
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

### 自动机文件的格式
```
<origin>
<end1> <end2> ... <endx>
<letter1> <letter2> ... <lettery>
<edge_number1> <letter11> <to11> ... <letter1n> <to1n>
...
<edge_numbern> <lettern1> <ton1> ... <letternn> <to1n>
```

例如（对应课本40页自动机）
节点编号从0开始，ε用"."表示
```
0
4 5 6
a b
2 a 5 b 2
2 a 6 b 2
2 a 0 b 4
2 a 3 b 5
2 a 6 b 2
2 a 3 b 0
2 a 3 b 1
```

### GEN/KILL文件格式
```
<g1> <g2> ... <gn>, <k1> <k2> ... <kn>
...
```
例如（对应习题8-13的GEN/KILL）
```
1 2, 10 6 11
3 4, 5 8
5, 4 8
6 7, 2 9 11
8 9, 4 5 7
10 11, 1 2 6
```

### USE/DEF文件格式
```
, B: 3 6 7 8 11; C: 3 4 5 6 8 11
B: 3; C: 3 4, A: 7; D: 5 10 11
C: 5; D: 5, D: 10 11
B: 6 7; C: 6; A: 7, E: 9; C: 3 4 5 8 10
B: 8; C: 8; E: 9, D: 5 10 11
C: 10; D: 10 11, B: 3 6 7 8; C: 3 4 5 6 8
```
对应习题8-13的USE/DEF
USE与DEF用","分隔，不同字母的引用点用";"分隔，不同引用点用空格分隔


### 图（程序控制流图）格式
```
<edge_count1> <to1> <to2> ... <ton>
...
<edge_countn> <to1> <to2> ... <ton>
```

例如
节点编号从0开始
```
1 1
2 2 3
2 3 4
2 5 1
1 2
0
```


### 功能
- 计算LR(0)项目集规范族
- 计算LR(1)项目集规范族
- 计算FIRST、FOLLOW集合
- 计算LR(0)、SLR(1)、LR(1)、LALR(1)分析表
- 执行LR(0)、SLR(1)、LR(1)、LALR(1)分析
- 计算LL(1)分析表
- 执行LL(1)分析
- 判断是否为LR(0)、SLR(1)、LR(1)、LALR(1)、LL(1)文法
- 非确定性自动机确定化
- 自动机最小化
- 根据GEN/KILL计算到达-定值方程
- 根据USE/DEF计算活跃变量方程
- 计算必经节点集与回边


### 使用举例
```
exam --help
exam --file grammar.txt --lr0f
exam --file grammar.txt --is-lalr1
exam --ud graph.txt gen_kill.txt
exam --du graph.txt use_def.txt
exam --nfa2dfa --file xxx.nfa
```