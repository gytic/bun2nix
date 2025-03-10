import LRU from "lru-cache";
import * as cacache from "cacache";

const cacheKey = "key";

const lruCache = new LRU({});
lruCache.set(cacheKey, "Hello from lru-cache\n");

const cacachePath = "./.cache/test-cacache";
cacache.put(cacachePath, cacheKey, "Hello from cacache\n");

cacache.get(cacachePath, cacheKey).then((cacacheValue) => {
    console.log(lruCache.get(lruCache) as string + cacacheValue);
});

