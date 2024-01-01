const removeIndexElement = (array: any[], index: number): any[] =>
  array.filter((_, i) => i !== index);

export default removeIndexElement;
