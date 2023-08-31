## 175. 组合两个表

> 编写解决方案，报告 Person 表中每个人的姓、名、城市和州。如果 personId 的地址不在 Address 表中，则报告为 null 。
> 以 任意顺序 返回结果表。

[组合两个表](https://leetcode.cn/problems/combine-two-tables/description/)



** Write your MySQL query statement below**

```sql
select p.FirstName, p.LastName, a.City, a.State
from Person p
left join Address a
on p.PersonId = a.PersonId
```
