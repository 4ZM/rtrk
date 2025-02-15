use std::io;
use std::io::Cursor;

struct DataModel {
    num: u8,
}

struct ViewModel {
    repeat: u8,
}
impl ViewModel {
    fn num(&self, dm: &DataModel) -> String {
        let mut n: String = String::new();

        for _ in 0..self.repeat as usize {
            n += format!("{}", dm.num).as_str();
        }

        n
    }
}

struct View {}
impl View {
    fn render<W: io::Write>(&self, w: &mut W, vm: &ViewModel, dm: &DataModel) {
        write!(w, "[{}]", vm.num(&dm)).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn exploration_test() {
        let mut buffer = Cursor::new(Vec::new());

        let dm: DataModel = DataModel { num: 23 };
        let vm: ViewModel = ViewModel { repeat: 2 };
        let view: View = View {};
        view.render(&mut buffer, &vm, &dm);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "[2323]");
    }
}
