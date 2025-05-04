# zms-tradition-data
zms-tradition-data 

## 组件

 clickhouse
 redis

## 设计思路
 1. 维护交易所(币安)历史数据[历史最后维护时间到现在,历史最远维护时间到过去;历史维护时间不存在就取当前时间到现在]
 2. 以ticker数据订阅事件更新最新k线数据 todo;弥补定时任务延时情况
3. 提供数据接口 时间范围 周期 币种名称,redis缓存机制 