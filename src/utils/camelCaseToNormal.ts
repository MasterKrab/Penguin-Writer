const camelCaseToNormal = (text: string) =>
  `${text.charAt(0).toUpperCase()}${text
    .slice(1)
    .replace(/[A-Z]/g, (match) => ` ${match.toLowerCase()}`)}`;

export default camelCaseToNormal;
