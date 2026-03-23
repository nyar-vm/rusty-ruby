# 聚合与统计

RBQ 的聚合语法摆脱了 SQL 中 `GROUP BY` 与 `SELECT` 字段必须严格对应的繁琐限制。它将分组视为一种**数据桶 (Bucket)** 操作。

## 1. 分组 (group_by)

使用 `group_by` 将数据流切分为多个小组。

```rbq
products.group_by { $category_id }
```

此时，数据流的每一项变成了 `{ $key, $group }`：
*   **$key**：分组的键值。
*   **$group**：该组内所有原始记录的集合。

## 2. 聚合转换

结合 `map` 算子对每个小组进行统计。

```rbq
products.group_by { $category_id }
        .map {
            category = $key
            total_count = $group.count()
            avg_price = $group.avg { $price }
            max_price = $group.max { $price }
        }
        .list();
```

## 3. 常用聚合函数

*   **count()**：计数。
*   **sum { expr }**：求和。
*   **avg { expr }**：平均值。
*   **min / max { expr }**：最小值/最大值。
*   **collect { expr }**：将组内某个字段收集为列表。

## 4. 过滤分组 (Having)

在 RBQ 中，由于它是流式处理，你只需要在聚合后的 `map` 之后再接一个 `filter` 即可实现 SQL 中的 `HAVING` 效果。

```rbq
products.group_by { $category_id }
        .map {
            category = $key
            count = $group.count()
        }
        .filter { $count > 5 } # 只保留产品数大于 5 的分类
        .list();
```

---

**小结**：
*   聚合是“先分组，后统计”的两步走。
*   `$group` 提供了强大的组内操作能力。
*   逻辑清晰，不再有 `GROUP BY` 的语法陷阱。

**下一节**：数据处理逻辑已经掌握，现在我们将进入工程组织篇，学习如何通过 `trait` 来复用代码。
