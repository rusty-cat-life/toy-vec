pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    // 説明のためにライフタイムを明示しているが、本当は省略できる
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            // Iter構造体の定義より、ライフタイムは'vecになる
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

// ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vec T型の値が
// 生存してる間は、ToyVecは変更できない
pub struct Iter<'vec, T> {
    // ToyVec構造体のelementsを指す不変の参照
    elements: &'vec Box<[T]>,
    // ToyVecの長さ
    len: usize,
    // 次に返す要素のインデックス
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型（トレイトに関連付いた型）で、このイテレータがイテレートする要素の型を指定する
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    // 要素があるなら不変の参照（&T）をSomeで包んで返し、ないときはNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    // イテレータがイテレートする値の型
    type Item = &'vec T;
    // into_iterメソッドの戻り値の型
    type IntoIter = Iter<'vec, T>;

    // &ToyVec<T>に対するトレイト実装なので、selfの型はToyVec<T>ではなく&ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// &ToyVec<T>にIntoIteratorを実装し、Iter<T>を返すようにしたので以下のように使える

// let mut v = ToyVec::new();
//     v.push("Hello, ");
//     v.push("World!\n");
// for msg in &v {
//     print(msg);
// }
