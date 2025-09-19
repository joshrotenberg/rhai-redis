# rhai-redis Design Notes

## Our Approach vs Idiomatic Rhai Packages

### Idiomatic Pattern (def_package!)
Most Rhai packages use the def_package! macro for stateless functions.

### Our Approach (Manual Registration)
We use manual registration because RedisClient is stateful (holds connection).

## Why This is Correct

The def_package! pattern works for stateless functions. 
Our Redis operations are stateful methods on a connection object.

What we do (CORRECT):
- redis.set("key", "value")
- redis.get("key") 

What def_package! would give (WRONG for us):
- redis_set(redis, "key", "value")
- redis_get(redis, "key")

## Conclusion

Our approach is correct for a stateful Redis client. Manual registration gives us the clean object-oriented API users expect.
