# Cols -- 一个快捷的字段选择器


## Cols的主要用法

有一程序输出结果如下所示:
|name1|name2|name3|
|-|-|-|
|0,0|0,1|0,2|
|1,0|1,1|1,2|
|2,0|2,1|2,2|

如果想选出去除表头后的第二第三列则使用如下命令
```
sh > cols -r 1 1,2
```
或
```
sh > cols -r 1 1-2
```

-r 选项后面表示表头的高度

用,分割表示单个值. 可以使用"1,2,5" 表示列出第2,3,6列

用-表示连续值. 如"1-6" 表示第2到第7列的所有列