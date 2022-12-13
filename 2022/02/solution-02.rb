#!/usr/bin/env ruby

# A, X: Rock     (1)
# B, Y: Paper    (2)
# C, Z: Scissors (3)
#
# A,X > C,Z
# C,Z > B,Y
# B,Y > A,X

SCORES = {
	"A X" => 4, # draw
	"A Y" => 8, # win
	"A Z" => 3, # lose
	"B X" => 1, # lose
	"B Y" => 5, # draw
	"B Z" => 9, # win
	"C X" => 7, # win
	"C Y" => 2, # lose
	"C Z" => 6, # draw
}

print File.read("input-02.txt").lines.map(&:strip).map{ |e| SCORES[e] }.sum()

