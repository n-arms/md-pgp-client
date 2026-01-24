export class Buffer {
  surrogates: number[] = [];
  end: number = 0;
  start: number = 0;
  startUnit: number = 0;
  endUnit: number = 0;
  length: number = 0;

  insert(text: string) {
    let i = 0;
    const items = [];
    while (i < text.length) {
      if (text.charCodeAt(i) == text.codePointAt(i)) {
        i++;
      } else {
        items.push(this.startUnit + i);
        i += 2;
      }
    }
    this.surrogates.splice(this.start, this.end - this.start);
    this.surrogates.splice(this.start, 0, ...items);
    this.length += text.length - (this.endUnit - this.startUnit);
    const cursor = this.start + items.length;
    this.end = cursor;
    this.start = cursor;
  }

  delete() {
    this.surrogates.splice(this.start, this.end - this.start);
    this.length -= this.endUnit - this.startUnit;
    this.end = this.start;
  }

  search(index: number) {
    const negative = index - this.length
    let l = 0;
    let r = this.surrogates.length;
    while (l < r) {
      const m = Math.floor(l + (r - l) / 2);
      const middle = this.surrogates[m];
      if (middle >= 0 && middle < index || middle < 0 && middle < negative) {
        l = m + 1;
      } else {
        r = m;
      }
    }
    return l;
  }

  unitToPointOffset(unitOffset: number, index: number) {
    let surrogate = this.surrogates[index];
    if (surrogate < 0) surrogate += this.length;
    if (surrogate < unitOffset) return unitOffset - index - 1;
    return unitOffset - index;
  }

  updateSelection(start: number, end: number) {
    this.startUnit = start;
    this.endUnit = end;
    const before = this.search(start);
    const newCursor = this.search(end);
    if (newCursor < this.end) {
      for (let i = newCursor; i < this.end; i++) {
        this.surrogates[i] -= this.length;
      }
    } else if (newCursor > this.end) {
      for (let i = this.end; i < newCursor; i++) {
        this.surrogates[i] += this.length;
      }
    }
    this.start = before;
    this.end = newCursor;
    return { start: this.unitToPointOffset(start, before), end: this.unitToPointOffset(end, newCursor) }
  }
}
