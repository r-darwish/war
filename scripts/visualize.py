#!/usr/bin/env python

import pandas # fades
import matplotlib.pyplot as plt # fades
import sys

def main():
    data = pandas.read_csv(sys.argv[1])
    red = plt.bar(data.index, data['red_player_cards'], 1, color='red')
    blue = plt.bar(data.index, data['blue_player_cards'], 1, color='blue', bottom=data['red_player_cards'])
    plt.ylabel('Number of cards')
    plt.xlabel('Turns')
    plt.legend((red[0], blue[0]), ('First player cards', 'Second player cards'))
    plt.title("War game - {} turns".format(len(data)))
    plt.show()

if __name__ == "__main__":
    sys.exit(main())
