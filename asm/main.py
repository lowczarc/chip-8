import sys

instructions = [
    { "opcode": "SYS", "params": [
        { "type": ["addr"], "format": "0nnn" }
    ] },
    { "opcode": "CLS", "params": [
        { "type": [], "format": "00E0"}
    ]},
    { "opcode": "RET", "params": [
        { "type": [], "format": "00EE" }
    ]},
    { "opcode": "JP", "params": [
        { "type": ["addr"], "format": "1nnn" },
        { "type": ["Vx", "addr"], "format": "Bnnn" }
    ] },
    { "opcode": "CALL", "params": [
        { "type": ["addr"], "format": "2nnn" }
    ] },
    { "opcode": "SE", "params": [
        { "type": ["Vx", "byte"], "format": "3xkk" },
        { "type": ["Vx", "Vx"], "format": "5xy0" }
    ] },
    { "opcode": "SNE", "params": [
        { "type": ["Vx", "byte"], "format": "4xkk" },
        { "type": ["Vx", "Vx"], "format": "9xy0" }
    ] },
    { "opcode": "LD", "params": [
        { "type": ["Vx", "byte"], "format": "6xkk" },
        { "type": ["Vx", "Vx"], "format": "8xy0" },
        { "type": ["I", "addr"], "format": "Annn" },
        { "type": ["Vx", "DT"], "format": "Fx07" },
        { "type": ["Vx", "K"], "format": "Fx0A" },
        { "type": ["DT", "Vx"], "format": "Fx15" },
        { "type": ["ST", "Vx"], "format": "Fx18" },
        { "type": ["F", "Vx"], "format": "Fx29" },
        { "type": ["B", "Vx"], "format": "Fx33" },
        { "type": ["[I]", "Vx"], "format": "Fx55" },
        { "type": ["Vx", "[I]"], "format" :"Fx65" },
    ] },
    { "opcode": "ADD", "params": [
        { "type": ["Vx", "byte"], "format": "7xkk" },
        { "type": ["Vx", "Vx"], "format": "8xy4" },
        { "type": ["I", "Vx"], "format": "Fx1E" },
    ] },
    { "opcode": "OR", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy1" },
    ] },
    { "opcode": "AND", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy2" },
    ] },
    { "opcode": "XOR", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy3" },
    ] },
    { "opcode": "SUB", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy5" },
    ] },
    { "opcode": "SHR", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy6" },
    ] },
    { "opcode": "SUBN", "params": [
        { "type": ["Vx", "Vx"], "format": "8xy7" },
    ] },
    { "opcode": "SHL", "params": [
        { "type": ["Vx", "Vx"], "format": "8xyE" },
    ] },
    { "opcode": "RND", "params": [
        { "type": ["Vx", "byte"], "format": "Cxkk" },
    ] },
    { "opcode": "DRW", "params": [
        { "type": ["Vx", "Vx", "nibble"], "format": "Dxyn" },
    ] },
    { "opcode": "SKP", "params": [
        { "type": ["Vx"], "format": "Ex9E" },
    ] },
    { "opcode": "SKNP", "params": [
        { "type": ["Vx"], "format": "ExA1" },
    ] },
    { "opcode": "DATA", "params": [
        { "type": ["16b"], "format": "dddd" },
    ] },
]

class Param:
    def __init__(self, value, labels):
        self.type, self.value = self.get_type_value(value.upper().strip(), labels)

    def get_type_value(self, input, labels):
        if len(input) == 2 and input[0] == 'V':
            return 'Vx', int(input[1], 16)
        elif len(input) == 3 and input[:2] == '0X':
            return 'nibble', int(input[2:], 16)
        elif len(input) == 4 and input[:2] == '0X':
            return 'byte', int(input[2:], 16)
        elif len(input) == 5 and input[:2] == '0X':
            return 'addr', int(input[2:], 16)
        elif len(input) == 6 and input[:2] == '0X':
            return '16b', int(input[2:], 16)
        elif input in ['I', 'DT', 'ST', 'B', 'F', '[I]']:
            return input, None
        elif input.startswith('=') and input[1:] in labels:
            return 'addr', labels[input[1:]]
        else:
            raise ValueError('Invalid parameter')

    def replace_format(self, format):
        if self.type == 'Vx':
            if 'x' in format:
                return format.replace('x', "{:01x}".format(self.value))
            else:
                return format.replace('y', "{:01x}".format(self.value))
        elif self.type == 'nibble':
            return format.replace('n', "{:01x}".format(self.value))
        elif self.type == 'byte':
            return format.replace('kk', "{:02x}".format(self.value))
        elif self.type == 'addr':
            return format.replace('nnn', "{:03x}".format(self.value))
        elif self.type == '16b':
            return format.replace('dddd', "{:04x}".format(self.value))
        else:
            return format

class Instruction:
    def __init__(self, value, labels):
        splitted = value.split(' ')
        self.opcode = splitted[0].upper().strip()
        self.params = [Param(param, labels) for param in splitted[1:]]

    def get_instruction_format(self):
        for instruction in instructions:
            if self.opcode == instruction['opcode']:
                for params in instruction['params']:
                    if len(params["type"]) == len(self.params):
                        for i in range(len(params["type"])):
                            if params["type"][i] != self.params[i].type:
                                break
                        else:
                            return params
                return None

    def format_instruction(self):
        instruction_format = self.get_instruction_format()['format']
        for param in self.params:
            instruction_format = param.replace_format(instruction_format)
        return instruction_format.upper()

    def __str__(self):
        return self.format_instruction()

    def to_bytes(self):
        first_byte = int(self.format_instruction()[:2], 16)
        second_byte = int(self.format_instruction()[2:], 16)

        return first_byte, second_byte

def main():
    if len(sys.argv) < 3:
        print("Usage: chip8-asm <filename.c8asm> <output.ch8>")
        return

    filename = sys.argv[1]

    f = open(filename, "r")

    starting_address = 0x200
    lines = []
    labels = {}
    # Preprocess
    for line in f:
        line_without_comment = line.split(';')[0].strip()

        if ':' in line_without_comment:
            splitted = line_without_comment.split(':')
            labels[splitted[0].strip()] = starting_address
            line_without_comment = splitted[1].strip()

        if line_without_comment != '':
            lines.append(line_without_comment)
            starting_address += 2

    program = []
    # Compile
    for line in lines:
        instruction = Instruction(line, labels)
        print("Instruction: " + instruction.opcode + " " + str(instruction.params))
        print("Valid: " + str(instruction.get_instruction_format()))
        print("Format: " + instruction.format_instruction())

        first_byte, second_byte = instruction.to_bytes()
        program.append(first_byte)
        program.append(second_byte)

    if len(sys.argv) > 2:
        output = open(sys.argv[2], "wb")
        output.write(bytearray(program))
        output.close()

if __name__ == "__main__":
    main()
