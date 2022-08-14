#!/bin/bash/env python3

price = 3950

for coin_500 in range(0, 11):
    for coin_100 in range(0, 4):
        for coin_50 in range(0, 11):

            total = coin_500 * 500 + coin_100 * 100 + coin_50 * 50
            if total == price:
                print(f"500円 * {coin_500} + 100円 * {coin_100} + 50円 * {coin_50} = {total}")
                break
            elif total > price:
                print(f"おつり: {total - price}円")
                break
            else:
                continue
