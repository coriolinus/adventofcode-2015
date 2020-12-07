
import sys

# Modes of operation
NORMAL = 0
ESCAPE = 1
HEX    = 2

def chars(line):
    return sum((1 for c in line))

def mem(line):
    if len(line) == 0: return ''
    if not (line[0] == '"' and line[-1] == '"'):
        raise Exception("Could not parse " + line)
    line = line[1:-1]

    out = ""

    hex_buffer = ""
    mode = NORMAL
    for char in line:
        if mode == NORMAL:
            if char == '\\':
                mode = ESCAPE
            else:
                out += char

        elif mode == ESCAPE:
            if char == '\\' or char == '"':
                mode = NORMAL
                out += char
            elif char == 'x':
                mode = HEX
                hex_countdown = 2
            else:
                raise Exception("Invalid escape sequence used: \\"+char)

        elif mode == HEX:
            hex_buffer += char
            if len(hex_buffer) == 2:
                mode = NORMAL
                out += chr(int(hex_buffer, 16))
                hex_buffer = ""
            else:
                if not char.lower() in 'abcdef1234567890':
                    raise Exception("Invalid hex sequence")

    return out

def encode(line):
    out = '"'
    for ch in line:
        if ch == '"' or ch == '\\':
            out += '\\' + ch
        else:
            out += ch
    out += '"'
    return out

def santas_desire(line):
    return chars(line) - len(mem(line))

def santas_desire2(line):
    return len(encode(line)) - chars(line)

def echo(lines):
    for line in lines.split('\n'):
        print(line)

if __name__ == '__main__':
    print("Your EOF-terminated lines go here: ")
    lines = sys.stdin.read()
    #lines = ['""', '"abc"', r'"aaa\"aaa"', r'"\x27"']
    #for line in lines:
    #    print(line, encode(line), chars(line), len(encode(line)), santas_desire2(line))
    #print(sum((santas_desire2(l) for l in lines)))

    print(sum((santas_desire(l) for l in lines.split('\n'))))
    print(sum((santas_desire2(l) for l in lines.split('\n') if len(l) > 0)))
