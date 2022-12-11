import fileinput


class CmdParser:

    path_separator = '#'

    def __init__(self, xs):
        self.xs = xs
        self.paths = {}
        self.path = []

    def _to_path(self, path):
        return self.path_separator.join(path)

    def _get_path(self):
        return self._to_path(self.path)

    def _ls(self):
        total_size = 0
        files = {}
        for line in self.xs:
            if line.startswith('$'):
                self.paths[self._get_path()] = sum(files.values())
                self._parse_command(line)
                return
            elif not line.startswith('dir'):
                size, name = line.split(' ', 1)
                files[name] = int(size)
        self.paths[self._get_path()] = sum(files.values())

    def _cd(self, line):
        dir_name = line[len('$ cd'):].strip()
        if dir_name == '/':
            self.path = ['/']
        elif dir_name == '..' and self.path:
            self.path.pop()
        else:
            self.path.append(dir_name)

    def _parse_command(self, line):
        assert '$' == line[0]
        if line.startswith('$ cd'):
            self._cd(line)
        elif line.startswith('$ ls'):
            self._ls()

    def parse(self):
        for x in self.xs:
            self._parse_command(x)

        for path in reversed(sorted(self.paths)):
            if path == '/':
                continue
            dir_names = path.split(self.path_separator)
            size = self.paths[path]
            self.paths[self._to_path(dir_names[:-1])] += size

xs = fileinput.input()
cmd = CmdParser(xs)
cmd.parse()
print(sum(x for x in cmd.paths.values() if x < 100000))
to_free = 30_000_000 - (70_000_000 - cmd.paths['/'])
print(min([size for size in cmd.paths.values() if size >= to_free]))

