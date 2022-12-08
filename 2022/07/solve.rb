#!/usr/bin/env ruby

def init_dir
    {dirs: {}, files: {}}
end

def cd(cwd, dir)
    cwd[:dirs].fetch(dir) do |d|
        cwd[:dirs][d] = init_dir()
    end
end

def get_dir(root, path)
    path.reduce(root) { |parent, dir| cd(parent, dir) }
end

def tree(dir, level=0)
    for k, v in dir[:dirs]
        print "  " * level
        puts "D " + k
        tree(v, level + 1)
    end
    for k, v in dir[:files]
        print "  " * level
        puts "F " + k + " " + v
    end
end

root = init_dir()

s = File
    .read('input.txt')
    .lines
    .map(&:strip)
    .map{|e| e.split(' ')}
    .reduce([root, []]) do |(root, path), cmd|
        case cmd
        in ["$", "cd", dir] if dir == "/"
            [root, []]
        in ["$", "cd", dir] if dir == ".."
            [root, path[...-1]]
        in ["$", "cd", dir]
            [root, path << dir]
        in [size, filename] if size.to_i > 0
            cwd = get_dir(root, path)
            cwd[:files][filename] = size.to_i
            [root, path]
        else
            [root, path]
        end
    end

$total_sum = 0
$candidate_sizes = []

def dir_size(dir)
    files = dir[:files]
    subdirs = dir[:dirs]
    total_size = files.values.sum + subdirs.values.map { |dir| dir_size(dir) }.sum
    if total_size <= 100000 then
        $total_sum += total_size
    end
    if total_size >= 532950 then
        $candidate_sizes << total_size
    end
    total_size
end

root_size = dir_size(root)
free_space = 70000000 - root_size
space_to_free = 30000000 - free_space

puts "Sol 1: #{$total_sum}"
puts "Sol 2: #{$candidate_sizes.min}"
