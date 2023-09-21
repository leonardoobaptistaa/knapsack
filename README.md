Knapsack
========

This project compares the same knapsack algorithm in python and rust.

## Dependencies

- Docker
- Docker-compose


## How to run

### Rust

```
docker-compose run knapsack-rust
```

### Python

```
docker-compose run knapsack-python
```

## Results

In my enviroment, to solve a knapsack problem with 240 inputs, rust is 30x faster than python.

Python 3.12-rc took 31,3 seconds.
```
docker-compose run knapsack-python

real    0m31,325s
```

Rust 1.72.1 took 1,19 second.
```
dcr --rm knapsack-rust

real    0m1.195s
```