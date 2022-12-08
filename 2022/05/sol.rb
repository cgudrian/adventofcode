#!/usr/bin/env ruby

input = File.read('input.txt').split("\n")

setup = input.shift(input.index(""))

# drop the last line
setup.pop(1)

# we expect 9 crates
crates = [[], [], [], [], [], [], [], [], []]

# fill the crates
while not setup.empty?
	line = setup.pop
	crates.each_with_index do |crate, i|
		item = line[1 + i*4].strip
		if not item.empty? then
			crate.append(item)
		end
	end
end

crates2 = crates.clone # for solution 2

input.shift(1) # remove empty line

moves = input
	.map{ |e| e.split(' ') }
	.map{ |e| [e[1], e[3], e[5]].map(&:to_i) }
	
moves.each do |num, from, to|
	for i in 1..num
		crates[to-1].push(crates[from-1].pop)
	end
end

crates.each do |crate|
	print crate, "\n"
end
print "\n"
