import _ from 'lodash';
import doT from 'dot';

const _firstUp = (str) => {
    return str
        .split('')
        .map((s, i) => i === 0 ? s.toLocaleUpperCase() : s)
        .join('');
};

const _camel = (str, firstUpper) => {
    let ret = str.toLowerCase();
    ret = ret.replace(/_([\w+])/g, function (all, letter) {
        return letter.toUpperCase();
    });
    if (firstUpper) {
        ret = ret.replace(/\b(\w)(\w*)/g, function ($0, $1, $2) {
            return $1.toUpperCase() + $2;
        });
    }
    return ret;
};

const _transform = (f, dataSource, code, type = 'id', codeType = 'dbDDL', omitName = []) => {
    // 获取该数据表需要显示的字段
    const domains = dataSource?.domains || [];
    const entities = dataSource?.entities || [];
    const mappings = dataSource?.dataTypeMapping?.mappings || [];
    const db = _.get(dataSource, 'profile.default.db', _.get(dataSource, 'profile.dataTypeSupports[0].id'));
    const dicts = dataSource?.dicts || [];
    const uiHints = _.get(dataSource, 'profile.uiHint', []);
    const temp = {};
    if (f.baseType) {
        const mapping = mappings.find(m => m.id === f.baseType);
        temp.baseType = mapping?.defName || mapping?.defKey || '';
        temp.type = mapping?.[code || db] || f.type || '';
        temp.dbType = mapping?.[db] || f.type || '';
        temp.baseTypeData = mapping;
    } else {
        temp.dbType = f.type || '';
    }
    if (f.domain) {
        // 转换数据域
        const domain = domains.find(dom => dom[type] === f.domain) || { len: '', scale: '' };
        temp.len = domain.len === undefined ? '' : domain.len;
        temp.scale = domain.scale === undefined ? '' : domain.scale;
        temp.domain = type === 'id' ? (domain.defName || domain.defKey) : f.domain;
        temp.domainData = domain;
    }
    // 转换数据字典
    if (f.refDict && !omitName.includes('refDict')) {
        const dictIndex = dicts.findIndex(d => d[type] === f.refDict);
        const dict = dicts[dictIndex];
        temp.refDict = dict?.defName || dict?.defKey;
        temp.refDictData = dict || {};
    }
    // 转换UI建议
    if (f.uiHint) {
        const uiHintIndex = uiHints.findIndex(u => u[type] === f.uiHint);
        if (uiHintIndex > -1) {
            const uiHint = uiHints[uiHintIndex];
            temp.uiHint = uiHint?.defName || uiHint?.defKey;
            temp.uiHintData = uiHint;
        }
    }
    // 转换引用数据表  如果是视图
    if (entities && f.refEntity) {
        const entity = entities.find(e => e[type] === f.refEntity);
        if (entity) {
            const field = (entity.fields || []).find(fie => f.refEntityField === fie[type]);
            temp.refEntity = entity.defKey || '';
            if (field) {
                temp.refEntityField = field?.defKey || '';
            } else {
                temp.refEntityField = '';
            }
        }
    }
    return temp;
};

const _getDefaultEnv = (e) => {
    return {
        ...(e.env || {}),
        base: {
            ...(e.env?.base || {}),
            nameSpace: e.env?.base?.nameSpace || '',
            codeRoot: e.env?.base?.codeRoot || _camel(e.defKey, true),
        }
    }
};

const _getTemplateString = (template, templateData, isDemo, dataSource, code) => {
    const underline = (str, upper) => {
        const ret = str?.replace(/([A-Z])/g, "_$1") || '';
        if (upper) {
            return ret.toUpperCase();
        } else {
            return ret.toLowerCase();
        }
    };
    const upperCase = (str) => {
        return str?.toLocaleUpperCase() || '';
    };
    const lowerCase = (str) => {
        return str?.toLocaleLowerCase() || '';
    };
    const join = (...args) => {
        if (args.length <= 2) return args[0];
        const datas = [];
        const delimter = args[args.length - 1];
        for (let i = 0; i < args.length - 1; i++) {
            if (/^\s*$/.test(args[i])) continue;
            datas.push(args[i]);
        }
        return datas.join(delimter);
    };
    const objectkit = {
        isJSON: function (obj) {
            var isjson = typeof (obj) == "object" && Object.prototype.toString.call(obj).toLowerCase() == "[object object]" && !obj.length;
            return isjson;
        },
        deepClone: function (obj) {
            return JSON.parse(JSON.stringify(obj));
        },
        equals: function (v1, v2) {
            if (typeof (v1) === "object" && objectkit.isJSON(v1) && typeof (v2) === "object" && objectkit.isJSON(v2)) {
                return JSON.stringify(v1) == JSON.stringify(v2);
            } else {
                return v1 == v2;
            }

        }
    };
    const getIndex = (array, arg, n) => {
        var i = isNaN(n) || n < 0 ? 0 : n;
        for (; i < array.length; i++) {
            if (array[i] == arg) {
                return i;
            } else if (typeof (array[i]) === "object" && objectkit.equals(array[i], arg)) {
                return i;
            }
        }
        return -1;
    };
    const contains = (array, obj) => {
        return getIndex(array, obj) >= 0;
    };
    const uniquelize = (array) => {
        var copy = clone(array);
        const temp = [];
        for (var i = 0; i < copy.length; i++) {
            if (!contains(temp, copy[i])) {
                temp.push(copy[i]);
            }
        }
        return temp;
    };
    const clone = (array) => {
        var cloneList = Array();
        for (var i = 0, a = 0; i < array.length; i++) {
            cloneList.push(array[i]);
        }
        return cloneList;
    };
    const each = (array, fn) => {
        fn = fn || Function.K;
        var a = [];
        var args = Array.prototype.slice.call(arguments, 1);
        for (var i = 0; i < array.length; i++) {
            var res = fn.apply(array, [array[i], i].concat(args));
            if (res != null) a.push(res);
        }
        return a;
    };
    const intersect = (array1, array2) => {
        // 交集
        const copy = clone(array1);
        const r = each(uniquelize(copy), function (o) { return contains(array2, o) ? o : null });
        return [].concat(r);
    };
    const union = (array1, array2) => {
        var copy = clone(array1);
        var r = uniquelize(copy.concat(array2));
        return [].concat(r);
    };
    const minus = (array1, array2) => {
        var copy = clone(array1);
        var r = each(uniquelize(copy), function (o) { return contains(array2, o) ? null : o });
        return [].concat(r);
    };
    const tplText = template.replace(/(^\s*)|(\s*$)/g, "");
    const getCode = () => {
        return code || _.get(dataSource, 'profile.default.db', dataSource.profile?.dataTypeSupports[0]?.id);
    }
    const getTemplate = () => {
        const allTemplate = _.get(dataSource, 'profile.codeTemplates', []);
        return allTemplate.filter(t => t.applyFor === getCode())[0] || {};
    };
    const currentEntityIndexRebuildDDL = (baseInfo, newIndexes = [], fields = [], type = 'entity') => {
        const codeTemplate = getTemplate();
        const data = isDemo ? demoTable.entity : { ...baseInfo, fields, indexes: newIndexes };
        return `${_getTemplateString(codeTemplate.deleteIndex || _getEmptyMessage('deleteIndex', dataSource, getCode()), {
            [type]: {
                ...data,
                env: _getDefaultEnv(data),
            },
            separator: templateData.sqlSeparator,
        })}${_getTemplateString(codeTemplate.createIndex || _getEmptyMessage('createIndex', dataSource, getCode()), {
            [type]: {
                ...data,
                env: _getDefaultEnv(data),
            },
            separator: templateData.sqlSeparator,
        })}`
    }
    const currentEntityDropDDL = (data, type = 'entity') => {
        const codeTemplate = getTemplate();
        return _getTemplateString(codeTemplate.deleteTable || _getEmptyMessage('deleteTable', dataSource, getCode()), {
            [type]: { defKey: isDemo ? demoTable.entity.defKey : data.defKey },
            type,
            separator: templateData.sqlSeparator,
        });
    };
    const currentEntityCreateDDL = (data, type = 'entity') => {
        const codeTemplate = getTemplate();
        const name = type === 'entity' ? 'createTable' : 'createView';
        return _getTemplateString(codeTemplate[name] || _getEmptyMessage(name, dataSource, getCode()), {
            [type]: isDemo ? demoTable.entity : {
                ...data,
                env: _getDefaultEnv(data),
            },
            separator: templateData.sqlSeparator,
        });
    }
    const conf = {
        evaluate: /\{\{([\s\S]+?)\}\}/g,
        interpolate: /\{\{=([\s\S]+?)\}\}/g,
        encode: /\{\{!([\s\S]+?)\}\}/g,
        use: /\{\{#([\s\S]+?)\}\}/g,
        define: /\{\{##\s*([\w\.$]+)\s*(\:|=)([\s\S]+?)#\}\}/g,
        conditional: /\{\{\?(\?)?\s*([\s\S]*?)\s*\}\}/g,
        iterate: /\{\{~\s*(?:\}\}|([\s\S]+?)\s*\:\s*([\w$]+)\s*(?:\:\s*([\w$]+))?\s*\}\})/g,
        varname: 'it',
        strip: false,
        append: true,
        doNotSkipEncoded: false,
        selfcontained: false
    };
    let resultText = doT.template(tplText, conf)({
        ...templateData,
        func: {
            camel: _camel,
            underline: underline,
            upperCase: upperCase,
            lowerCase: lowerCase,
            join: join,
            intersect: intersect,
            union: union,
            minus: minus,
            indexRebuildDDL: currentEntityIndexRebuildDDL,
            dropDDL: currentEntityDropDDL,
            createDDL: currentEntityCreateDDL,
        }
    });
    resultText = resultText.replace(/\n(\n)*( )*(\n)*\n/g, "\n");  //删除空行
    resultText = resultText.replace(/\r\n(\r\n)*( )*(\r\n)*\r\n/g, "\r\n"); //(不同操作系统换行符有区别)删除空行
    resultText = resultText.replace(/\$blankline/g, '');              //单独处理需要空行的情况
    return resultText;
};

export const _getAllDataSQLByFilter = (data, code, filterTemplate, filterDefKey) => {
    // 获取项目的一些配置信息
    const getDataSourceProfile = (data) => {
        const dataSource = { ...data };
        const datatype = _.get(dataSource, 'dataTypeMapping.mappings', []);
        const allTemplate = _.get(dataSource, 'profile.codeTemplates', []);
        const sqlSeparator = _.get(dataSource, 'profile.sql.delimiter', ';') || ';';
        return {
            dataSource,
            datatype,
            allTemplate,
            sqlSeparator
        };
    };
    // 获取全量脚本（删表，建表，建索引，表注释）
    const { dataSource, allTemplate, sqlSeparator } = getDataSourceProfile(data);
    const entities = dataSource.entities || [];
    const getTemplate = (templateShow) => {
        return allTemplate.filter(t => t.applyFor === code)[0]?.[templateShow] || '';
    };
    const getFilterData = (name) => {
        return (dataSource[name] || [])
            .filter(e => {
                if (filterDefKey) {
                    return (filterDefKey[name] || []).includes(e.id);
                }
                return true;
            })
            .map(e => ({
                ...e,
                datatype: name,
                groupType: `ref${_firstUp(name)}`
            }));
    };
    let sqlString = '';
    try {
        const tempData = code === 'dictSQLTemplate' ? getFilterData('dicts') : getFilterData('entities')
            .concat(getFilterData('views'));
        sqlString += tempData
            .map(e => {
                const tempTemplate = [...filterTemplate];
                let tempData = '';
                let data;
                if (code === 'dictSQLTemplate') {
                    data = {
                        dict: _.omit(e, ['groupType', 'datatype']),
                    }
                } else {
                    const name = e.datatype === 'entities' ? 'entity' : 'view';
                    const childData = {
                        ..._.omit(e, ['groupType', 'datatype']),
                        env: _getDefaultEnv(e),
                        fields: (e.fields || []).map(field => {
                            return {
                                ...field,
                                ..._transform(field, dataSource, code)
                            }
                        }),
                        indexes: (e.indexes || []).map(i => {
                            return {
                                ...i,
                                fields: (i.fields || []).map(f => {
                                    const field = (e.fields || []).find(ie => f.fieldDefKey === ie.id);
                                    return {
                                        ...f,
                                        fieldDefKey: field?.defKey || '',
                                    };
                                })
                            }
                        }),
                        correlations: (e.correlations || []).map(c => {
                            const refEntityData = entities.find(r => r.id === c.refEntity);
                            if (refEntityData) {
                                return {
                                    ...c,
                                    myField: (e.fields || []).find(field => field.id === c.myField)?.defKey,
                                    refEntity: refEntityData?.defKey,
                                    refField: (refEntityData.fields || []).find(field => field.id === c.refField)?.defKey,
                                }
                            }
                            return null
                        }).filter(e => !!e)
                    };
                    if (name === 'view') {
                        childData.refEntities = dataSource?.entities
                            ?.filter(e => childData?.refEntities.includes(e.id))
                            ?.map(e => e.defKey);
                    }
                    data = {
                        entity: childData,
                        view: childData,
                    }
                }
                const templateData = {
                    ...data,
                    group: (dataSource.viewGroups || [])
                        .filter(g => (g[e.groupType] || []).includes(e.id))
                        .map(g => _.pick(g, ['defKey', 'defName'])),
                    separator: sqlSeparator
                };
                if (tempTemplate.includes('createTable')) {
                    tempTemplate.push('createView');
                }
                tempTemplate.filter(t => {
                    if (e.datatype === 'entities') {
                        return t !== 'createView';
                    }
                    return t !== 'createTable';
                }).forEach(f => {
                    const code = `${_getTemplateString(getTemplate(f), templateData)}`;
                    tempData += code ? `${code}\n` : '';
                });
                return tempData;
            })
            .join('');
    } catch (e) {
        sqlString = JSON.stringify(e.message);
    }
    return sqlString;
};

export const getCode = (template, select, db, defaultDb, dataSource) => {
    return _getAllDataSQLByFilter(
        dataSource,
        db || defaultDb,
        template,
        select,
    )
};
