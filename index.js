var FFI = require('ffi');

var lib = FFI.Library('./target/debug/liblibshingles.so', {
    'count_same': [ 'float', [ 'string', 'string', 'int' ] ]
});

module.exports = lib.count_same;


var text1 = "<h>Сегодня</h> после аппаратного совещания, где был заслушан доклад министра энергетики, жилищно-коммунального хозяйства и государственного регулирования тарифов Удмуртии Ивана Маринина о подготовке к  следующему отопительному сезону, глава республики Александр Соловьев прокомментировал ситуацию с пуском тепла в этом году, сообщает пресс-служба главы и правительства Удмуртии. «У меня есть информация, как Ижевск готовился к отопительному сезону в 2010-м, в 2011-м и других годах, в том числе и нынче. Меня такая работа не устраивает, — сказал Александр Соловьев. — Много вопросов к Министерству жилищно-коммунального хозяйства, есть вопросы к заместителю председателя правительства Сивцову, который отвечает за ЖКХ. Это их работа — пусть разбираются». По словам руководителя региона, долги, из-за которых в том числе в Ижевске были проблемы с пуском тепла, влияют не только на отопительный сезон. Александр Соловьев сказал, что, имея большие долги, невозможно решать вопросы в Москве ни по дальнейшей газификации республики, ни по строительству плавательного бассейна. «Мне всегда говорят, что так не только у нас, но и в других субъектах. Не надо равняться на плохие примеры, — отметил глава Удмуртии. — Руководство городов обновили. Узкие места нам известны. Теперь при принятии бюджетов городов, бюджета республики необходимо учесть все ошибки, чтобы следующий отопительный сезон открыть безболезненно и своевременно. От каждого ответственного за тот или иной участок мне нужна эффективная работа — только и всего».";
var text2 = "Отвечая на вопрос представителей СМИ об использовании десяти новых автомобилей «Лада Веста», переданных в субботу республике Ижевским автозаводом, руководитель Удмуртии отметил, что эти машины получат районы республики, сообщает пресс-служба главы и правительства республики. Распределение произведено с учётом достижений той или иной территории и состояния используемых ныне автомобилей. «У некоторых глав уже совсем старые машины. Если бы было 25 автомобилей, я бы всем отдал. В бюджете у нас на приобретение транспорта было заложено 50 миллионов рублей, но я принял решение нынче не закупать новые машины», — сказал Александр Соловьев. «Лады Весты» будут переданы в распоряжение Алнашского, Балезинского, Игринского, Киясовского, Красногорского, Малопургинского, Селтинского, Сюмсинского, Юкаменского и Ярского районов.";

var text3 = '\n<article>\n            <div id="news-id-64544" style="display:inline;"><p>Многие ижевчане удивились и расстроились, почему самый главный фонтан, который находится на Центральной площади у Театра оперы и балеты, не работает. Мы обратились с этим вопросом к Администрации города Ижевска. И как нам сообщил Первый заместитель Главы Администрации города Ижевска Ильдар Бикбулатов, музыкально-световой фонтан был отключен с 30 июля по 10 августа для проведения профилактических работ на насосном оборудовании.</p>\n<p>После этого проводилась уборка и мойка чаши фонтана, и 11 августа фонтан заполнили водой. А с понедельника, 15 августа, одна из главных красот и достопримечательностей Ижевска заработала в полную силу. Однако, по ночам фонтан будет работать в, так называемом, «ночном режиме», то есть напор воды будет в два раза меньше.</p>\n<p>Также, в Управлении благоустройства и транспорта нам сообщили, что фонтан продолжит радовать ижевчан до 5 октября, естественно, в зависимости от температурных условий.</p></div>\n</article>\n\n\n\n<!-- последняя script async type="text/javascript">(function(w,doc) {\nif (!w.__utlWdgt ) {\n    w.__utlWdgt = true;\n    var d = doc, s = d.createElement(\'script\'), g = \'getElementsByTagName\';\n    s.type = \'text/javascript\'; s.charset=\'windows-1251\'; s.async = true;\n    s.src = (\'https:\' == w.location.protocol ? \'https\' : \'http\')  + \'://w.uptolike.com/widgets/v1/uptolike.js\';\n    var h=d[g](\'body\')[0];\n    h.appendChild(s);\n}})(window,document);\n</script>\n<div data-background-alpha="0.0" data-buttons-color="#FFFFFF" data-counter-background-color="#ffffff" data-share-counter-size="12" data-top-button="false" data-share-counter-type="disable" data-share-style="1" data-mode="share_picture" data-like-text-enable="false" data-mobile-view="true" data-icon-color="#ffffff" data-orientation="vertical" data-text-color="#000000" data-share-shape="round" data-sn-ids="vk.tw.ok." data-share-size="30" data-background-color="#ffffff" data-preview-mobile="false" data-mobile-sn-ids="vk.tw.wh.ok." data-pid="1442307" data-counter-background-alpha="1.0" data-following-enable="false" data-exclude-show-more="false" data-selection-enable="false" class="uptolike-buttons" ></div-->\n\n       ';
var text4 = '\n<article>\n            <div id="news-id-64542" style="display:inline;"><p>В группу «<strong><a href="https://vk.com/udmurtiya18rus" target="_blank">ИГГС</a></strong>» «ВКонтакте» один из пользователей выложил видео, на котором ижевские школьники ползают по стометровой вышке, которая стоит в центре города. Забрались они туда совершенно беспрепятственно.</p>\n<p>На видео видно, что они забрались почти на самый верх. Вышка находится во дворе домов, у перекрестка Пушкинской и Лихвинцева (напротив здания Правительства и резиденции Главы республики).</p>\n<p>– Я просто царь этой горы, я просто покорю эту вышку, – говорит один из школьников, а второй повисает в воздухе, держась за ограждение. Видно, что на подростках нет никаких средств страховки.</p>\n<p>Одному из мальчишек во время съемки позвонила «тетя Лена», но подросток ответил ей, что «гуляет в центре города».</p>\n<p>Есть версия, что школьники в этом видео хотя подражать «руферам», которые ползают по крышам различных зданий и сооружений. Руферство – это разновидность сталкерства, когда молодые люди проникают и исследуют разные труднодоступные объекты.</p>\n<p>Видео прислал Данил Колединов.</p>\n<p><!--il_iframe_begin://vk.com/video_ext.php?oid=309180423&amp;id=456239044&amp;hash=fd3be5d6535c2eb7&amp;hd=2--><!--il_iframe_end--></p></div>\n</article>\n\n\n\n<!-- последняя script async type="text/javascript">(function(w,doc) {\nif (!w.__utlWdgt ) {\n    w.__utlWdgt = true;\n    var d = doc, s = d.createElement(\'script\'), g = \'getElementsByTagName\';\n    s.type = \'text/javascript\'; s.charset=\'windows-1251\'; s.async = true;\n    s.src = (\'https:\' == w.location.protocol ? \'https\' : \'http\')  + \'://w.uptolike.com/widgets/v1/uptolike.js\';\n    var h=d[g](\'body\')[0];\n    h.appendChild(s);\n}})(window,document);\n</script>\n<div data-background-alpha="0.0" data-buttons-color="#FFFFFF" data-counter-background-color="#ffffff" data-share-counter-size="12" data-top-button="false" data-share-counter-type="disable" data-share-style="1" data-mode="share_picture" data-like-text-enable="false" data-mobile-view="true" data-icon-color="#ffffff" data-orientation="vertical" data-text-color="#000000" data-share-shape="round" data-sn-ids="vk.tw.ok." data-share-size="30" data-background-color="#ffffff" data-preview-mobile="false" data-mobile-sn-ids="vk.tw.wh.ok." data-pid="1442307" data-counter-background-alpha="1.0" data-following-enable="false" data-exclude-show-more="false" data-selection-enable="false" class="uptolike-buttons" ></div-->\n\n        '

console.log(+new Date());
lib.count_same.async(text3, text4, 2, function(err, res) {
    if(err) {
        return console.log('err', err);
    }
    console.log(res);
    console.log(+new Date());
});


