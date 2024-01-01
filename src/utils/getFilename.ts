const getFilename = (path: string) => path.match(/\/([^\/]+)$/)![1];

export default getFilename;
