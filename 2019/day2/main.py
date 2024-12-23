import logging


logger = logging.getLogger("intcode")


class IntcodeVm(object):
    """
    Intcode computer Virtual Machine
    """

    def __init__(self, program: str) -> None:
        self.program = list(map(int, program.split(",")))

        self._pc = 0

    def execute(self, noun: int = None, verb: int = None) -> int:
        if len(self.program) == 0:
            raise RuntimeError("Unable to execute empty program")

        if noun is not None:
            self.program[1] = noun
        if verb is not None:
            self.program[2] = verb

        opcode = self.program[self._pc]
        while opcode != 99:
            if self._pc > len(self.program) - 1:
                logger.warning("Walked off the end of the program")
                break

            if opcode not in {1, 2}:
                logger.warning(f"Unknown opcode {opcode}")
            else:
                if self._pc + 4 > len(self.program):
                    logger.error(
                        "Got to an instruction, but don't have stuff to operate on"
                    )

                pos1 = self.program[self._pc + 1]
                pos2 = self.program[self._pc + 2]
                result_pos = self.program[self._pc + 3]

                if opcode == 1:
                    self.program[result_pos] = self.program[pos1] + self.program[pos2]
                elif opcode == 2:
                    self.program[result_pos] = self.program[pos1] * self.program[pos2]

            self._pc += 4
            opcode = self.program[self._pc]

        return self.program[0]


def main() -> None:
    with open("input.txt") as f:
        program = f.read()

    # Special replacement per instructions
    vm = IntcodeVm(program)
    vm.program[1] = 12
    vm.program[2] = 2
    vm.execute()
    print(f"Part 1 solution: {vm.program[0]}")

    # Part 2
    # Program positions 1 and 2 are the program inputs, output is at position 0
    desired = 19690720
    found = False
    for noun in range(0, 100):
        for verb in range(0, 100):
            vm = IntcodeVm(program)
            result = vm.execute(noun, verb)
            if result == desired:
                found = True
                break
        if found:
            break

    if found:
        print(f"Found result with noun={noun} verb={verb}")
    else:
        print("Unable to find result")


if __name__ == "__main__":
    main()
