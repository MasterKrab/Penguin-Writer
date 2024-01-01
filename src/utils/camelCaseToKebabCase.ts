const camelCaseToKebabCase = (text: string) =>
  text.replace(/[A-Z]/g, (match) => `-${match.toLowerCase()}`);

export default camelCaseToKebabCase;
