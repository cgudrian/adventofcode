#!/usr/bin/env ruby

data = File.read('input.txt')

# Part One

for i in 0..data.length-1
	if data[i,4].chars.uniq.length == 4 then
		puts i+4
		break
	end
end

# Part Two

for i in 0..data.length-1
	if data[i,14].chars.uniq.length == 14 then
		puts i+14
		break
	end
end
