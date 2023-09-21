def knapsack(cap, weights, n):
    if (n == 0) or (cap == 0):
        return 0

    evaluating = weights[n - 1]
    if evaluating > cap:
        return knapsack(cap, weights, n - 1)

    prev_result = evaluating + knapsack(cap - evaluating, weights, n - 1)
    result = knapsack(cap, weights, n - 1)
    return max(prev_result, result)


gallon_size = 5
bottles = [1.0, 3.0, 3.5, 1.5] * 60
print("initialized")
weights = bottles
cap = gallon_size
n = len(bottles)

result = knapsack(cap, weights, n)
print(result)
