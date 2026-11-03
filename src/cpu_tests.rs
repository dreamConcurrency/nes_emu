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
