import _ from 'lodash';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import FsExtra from 'fs-extra';
import { getCode } from './utils.js';

// 读取PDManer数据源文件
const args = yargs(hideBin(process.argv))
  .command('pdgen', 'PDManer SQL generation')
  .option('source', {
    alias: 's',
    describe: 'Source PDManer JSON path'
  })
  .option('target', {
    alias: 't',
    describe: 'Target SQL path'
  })
  .parse()
const dataSource = FsExtra.readJSONSync(args.source, { encoding: 'utf-8', throws: false }) || {};

// 生成数据
const dataTypeSupports = _.get(dataSource, 'profile.dataTypeSupports', []);
const defaultDb = _.get(dataSource, 'profile.default.db', dataTypeSupports[0]?.id);
const codeTemplates = _.get(dataSource, 'profile.codeTemplates', []);
const dataTypeSupport = codeTemplates.filter(c => c.applyFor === defaultDb)[0];
const template = ['createTable', 'createIndex'];
const sql = getCode(template, null, dataTypeSupport.applyFor, defaultDb, dataSource);

// 写入SQL文件
FsExtra.outputFileSync(args.target, sql, { encoding: 'utf-8' });
