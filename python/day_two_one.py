import re
import argparse
from dataclasses import dataclass

COLORS = ["red", "green", "blue"]
NUMBER_OF_CUBES = {"red": 12, "green": 13, "blue": 14}

# https://adventofcode.com/2023/day/2

# NEED TO FIX PARSING, ISN'T CATCHING Game 2
@dataclass()
class Hand:
    """
    A list of hands constitutes a single game; if a color is zero then it's not included in the hand
    """
    red: int = 0
    green: int = 0
    blue: int = 0

def solve(path: str) -> int:
    """
    Read in the data from whatever file we spec, trim and turn each hand into a Hand object; return list containing a list of hands
    ---
    Args:
        path (str): File path for the data set that we want to read (either example or puzzle for AOC)
    Returns:
        list: List of all games played in the puzzle with trimmed format (ex. [['3 blue, 4 red', '1 red, 2 green, 6 blue', '2 green'], ...])
    """
    sum_of_game_ids = 0
    with open(path, "r") as f:
        # Each line represents a "game"; ind. hands in a game are delineated by semicolons
        games = []
        for line in f:
            game_number = int(re.search(r"(?P<game_number>Game \d+)", line).group("game_number").strip("Game "))
            valid_game = True
            if game_number == 1:
                print("---")
            print("Game {}".format(game_number))
            # trim string for each hand, split the hand into a list by semicolon, trim whitespace
            hands = [
                h.strip()
                for h in re.sub("Game \d+\:\s", "", line).replace("\n", "").split(";")
            ]
            # Now apply a split for each of the hands in a game
            for i in hands:
                # split each hand into ind. marble counts, trim to remove remaining whitespace
                temp = [j.strip() for j in i.split(",")]
                unvalidated_hand = make_hand(temp)
                print(unvalidated_hand)
                if check_hand_validity(unvalidated_hand) == False:
                    valid_game = False
                    print(unvalidated_hand, "yields an invalid game!")
                    break

            if valid_game == True:
                sum_of_game_ids += game_number
    
    return sum_of_game_ids

def check_hand_validity(h: Hand) -> bool:
    """
    This function will determine whether or not a hand is valid according to the number of marbles that the elf has specified
    ---
    Args:
        h (Hand): The hand that we want to validate
    Returns:
        bool: True if the hand is valid, False if not
    """
    global NUMBER_OF_CUBES
    valid = True
    for i in NUMBER_OF_CUBES.keys():
        if getattr(h, i) >= NUMBER_OF_CUBES.get(i):
            valid = False
    return valid


def make_hand(l: list) -> Hand:
    """
    Given a list of marble counts (i.e. ['3 blue', '4 red']) generate a Hand object
    ---
    Args:
        l: list of marge counts
    Returns:
        Hand: Corresponding hand object
    """
    global COLORS
    h = Hand()
    for color in COLORS:
        for i in l:
            if color in i:
                color_re = r"\s" + color + r"\s?"
                # Get the count for each color from the string, strip whitespace to be safe
                marble_count = re.sub(color_re, "", i).strip()
                setattr(h, color, int(marble_count))
    return h

def main():
    parser = argparse.ArgumentParser(
                    prog='AOC_Day_Two')
    group = parser.add_mutually_exclusive_group()
    group.add_argument("-e", action='store_true')
    group.add_argument("-p", action='store_true')
    args = parser.parse_args()
    if args.e == True:
        example = solve(path = "../data/day_two/example_one.txt")
    if args.p == True:
        example = solve(path = "../data/day_two/puzzle_one.txt")
    print("Valid games", example)

if __name__ == "__main__":
    main()
