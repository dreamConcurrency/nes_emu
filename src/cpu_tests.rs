use super::cpu::CPU;

#[test]
fn test_tax() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;

    cpu.load_and_run(vec![0xaa, 0x00]);
    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_txa() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;
    
    cpu.load_and_run(vec![0x8a, 0x00]);
    assert_eq!(cpu.register_a, 10);
}

#[test]
fn test_tay() {
    let mut cpu = CPU::new();
    cpu.register_a = 5;

    cpu.load_and_run(vec![0xa8, 0x00]);
    assert_eq!(cpu.register_y, 5);
}

#[test]
fn test_tya() {
    let mut cpu = CPU::new();
    cpu.register_y = 5;

    cpu.load_and_run(vec![0x98, 0x00]);
    assert_eq!(cpu.register_a, 5);
}

#[test]
fn test_dey() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;

    cpu.load_and_run(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 0);
}

#[test]
fn test_iny() {
    let mut cpu = CPU::new();
    cpu.register_y = 1;

    cpu.load_and_run(vec![0xc8, 0x00]);
    assert_eq!(cpu.register_y, 2);
}
