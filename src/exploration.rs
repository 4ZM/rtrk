//struct Model {}

use std::io;
use std::io::Cursor;

struct DataModel {
    num: u8,
}

struct ViewModel<'a> {
    dm: &'a DataModel,
    repeat: u8,
}
impl<'a> ViewModel<'a> {
    fn new(dm: &'a DataModel, repeat: u8) -> Self {
        ViewModel { dm, repeat }
    }

    fn num(&self) -> String {
        let mut n: String = String::new();

        for _ in 0..self.repeat as usize {
            n += format!("{}", self.dm.num).as_str();
        }

        n
    }
}

struct View<'a> {
    vm: &'a ViewModel<'a>,
}
impl View<'_> {
    fn render<W: io::Write>(&self, w: &mut W) {
        write!(w, "[{}]", self.vm.num()).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn exploration_test() {
        let mut buffer = Cursor::new(Vec::new());

        let dm: DataModel = DataModel { num: 23 };
        let vm: ViewModel = ViewModel::new(&dm, 2);
        let view: View = View { vm: &vm };
        view.render(&mut buffer);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "[2323]");
    }
}
