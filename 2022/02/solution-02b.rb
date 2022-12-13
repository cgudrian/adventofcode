#!/usr/bin/env ruby

# A: Rock     (1)
# B: Paper    (2)
# C: Scissors (3)
#
# A > C
# B > A
# C > B
#
# X: lose
# Y: draw
# Z: win

SCORES = {
	"A X" => 3+0, # C
	"A Y" => 1+3, # A
	"A Z" => 2+6, # B
	"B X" => 1+0, # A
	"B Y" => 2+3, # B
	"B Z" => 3+6, # C
	"C X" => 2+0, # B
	"C Y" => 3+3, # C
	"C Z" => 1+6, # A
}

print File.read("input-02.txt").lines.map(&:strip).map{ |e| SCORES[e] }.sum()

