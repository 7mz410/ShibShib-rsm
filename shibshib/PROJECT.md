# ShibShib — ملف المشروع (اقرأه أولاً)

آخر تحديث: 2026-10-10 (بعد جلسة البرينستورم). هذا الملف هو نقطة البداية لأي جلسة جديدة. اقرأه بدل إعادة استكشاف المشروع، وحدّثه في نهاية كل جلسة.

## 1. الفكرة

**شبشب (ShibShib)**: حزمة برامج تصميم مجانية ومفتوحة المصدر، بديلة لبرامج Adobe، بواجهة عربية وإنجليزية. صاحب المشروع حمزة أبو عياش، مصمم يعمل على برامج Adobe منذ 2004. الهدف لاحقاً ربطها بموقع the-247.com.

| البرنامج | بديل | الأساس | الحالة |
|---|---|---|---|
| **rsm** (رسم) | Illustrator | fork من VectorCraft | ✅ شغّال، والواجهة العربية جارية |
| tlween (تلوين) | Photoshop | PhotoCraft | لاحقاً |
| trteeb (ترتيب) | InDesign | DesignCraft (لسا upstream بيتطور) | لاحقاً |
| tsweer (تصوير) | Premiere | FilmCraft | لاحقاً |
| effectat (تأثيرات) | After Effects | EffectCraft | لاحقاً |
| 7rrek (حرّك) | Animate/Flash | **نبنيه فوق rsm**: timeline، tweens، frame by frame | بعد rsm مباشرة |
| sharek (شارك) | Figma | Penpot على الأغلب، ويحتاج سيرفر | لاحقاً |
| ttshat (تحشات) | Lightroom | LightCraft (من نفس عائلة ArtCraft) | سهل، بعد 7rrek |
| mzika (مزيكا) | FL Studio | LMMS (GPL) | لاحقاً: مقامات بربع تون، آلات وإيقاعات شرقية |
| aswat (أصوات) | Audition | Audacity/Tenacity (GPL) أو SoundCraft، يحتاج فحصاً | لاحقاً: تحويل الكلام العربي لنص |

اللوغوهات (SVG) موجودة في `docs/shibshib/suite/`، وأصلها في `~/Documents/ShibShib/logos`.

## 2. الأماكن المهمة

- **الريبو:** https://github.com/7mz410/ShibShib-rsm، وهو fork لـ `storytold/vectorcraft`.
  - `origin` هو ريبونا، و`upstream` هو ArtCraft.
- **النسخة المحلية:** `~/Documents/ShibShib/rsm`
- **الويب:** https://7mz410.github.io/ShibShib-rsm/ من فرع `gh-pages`، ويُرفع يدوياً (انظر §6).
- **ملفاتنا داخل الريبو كلها في `shibshib/`:**
  - `PROJECT.md`: هذا الملف.
  - `CHANGES.md`: ما غيّرناه عن upstream، وهو مطلوب لرخصة Apache.
  - `rebrand.py`: يطبّق الاسم والشعار. شغّله بعد كل merge من upstream.
  - `ar-work/`: ملفات الترجمة على دفعات. `assemble.py` يبني منها `crates/ui-egui/src/i18n/ar.tsv`، و`check.py` يفحص كل دفعة.
- **الكود العربي:**
  - `crates/ui-egui/src/i18n/ar.tsv`: الترجمة.
  - `crates/ui-egui/src/i18n/bidi.rs`: ترتيب الكلمات من اليمين لليسار.
  - `plural_arabic` في `i18n/mod.rs`: قواعد الجمع العربية.
  - خط IBM Plex Sans Arabic في `assets/fonts`.
- **المشروع القديم (React):** محذوف. لا ترجع له.

## 3. القواعد الثابتة (مهمة)

1. **الرخصة والـ credits:**
   - نحافظ على `LICENSE-MIT` و`LICENSE-APACHE` و`NOTICE`، وعلى حقوق "ArtCraft Team and the VectorCraft contributors".
   - شكر VectorCraft موجود في الـ README وشاشة About.
   - ممنوع استخدام شعارات ArtCraft أو اسمهم كهوية لنا.
2. **تغيير الاسم الآن للنصوص الظاهرة فقط.** أسماء الـ crates وصيغة `.vectorcraft` تبقى كما هي، حتى يبقى الـ merge من upstream سهلاً. حمزة قرر أن نغيّر كل شيء **لاحقاً**، لا الآن.
3. **الـ commits:**
   - باسم `Hamza Abu Ayyash <hamza.abu3ayash@gmail.com>` فقط.
   - **ممنوع إضافة Co-Authored-By Claude**، لأن بعض الخدمات ترفضه.
4. **التوكنات:** لا نخزّن أي توكن في ملف. `gh` مسجّل دخول على جهاز حمزة.
5. **الخطوط:** نستعمل Google Fonts بدل Adobe Fonts.
6. **الكتب المرجعية** (PDF وEPUB) محمية بحقوق نشر، ولا تُرفع للريبو أبداً.
7. **الفحوصات قبل كل commit:**
   - `cargo xtask assets`: كل ملف asset له سطر في `ASSETS.md`.
   - `cargo xtask brands`: لا أسماء شركات في نص الواجهة.
8. **التواصل:** حمزة يكتب بالعربي العامي، والرد يكون بنفس اللغة، مختصراً ومباشراً.

## 4. وين كنا ← وين صرنا

1. بدأنا بمحرر React كتبناه بأنفسنا (VectorForge ثم ShibShib)، ثم أوقفناه.
2. اكتشفنا ArtCraft (برامج بلغة Rust، رخصتها MIT/Apache) وقررنا عمل fork.
3. **rsm:**
   - تم: fork، وتغيير الاسم، والأيقونة، والـ credits، ونسخة Mac (`dist/ShibShib rsm.app`)، ونسخة ويب.
   - تم: اللغة العربية بالكامل، 3652 جملة. الحروف تتصل صح، وترتيب الكلمات صحيح.
4. **آخر نقطة (2026-10-10):**
   - تصلّح اختبار قائمة اللغات ("English" يبقى بالإنجليزية).
   - الأرقام مثل 1920×1080 صارت صحيحة في النص العربي.
   - انشال زر Discord ورابط ArtCraft من الواجهة.
   - الويب محدّث بالواجهة العربية الكاملة.
   - صارت عندنا 4 سكيلز خاصة في `.claude/skills/` (انظر §8).
   - الاختبار الوحيد الفاشل هو `save_a_copy_and_template_suggest_their_names`، ويفشل في upstream نفسه.
   - لم ندمج تحديثات upstream بعد (استعمل `shibshib-sync`).

## 4.5 قرارات البرينستورم (2026-10-10)

- **الخطّاط (لوحة الخط العربي داخل rsm)** هي الإضافة الحقيقية لشبشب، وتأتي بعد RTL مباشرة. فيها:
  - كشيدة تمتد بالسحب، وبدائل الحروف، وتحريك التشكيل.
  - التركيب على طريقة الثلث والديواني، وقوالب تكوينات.
  - التنفيذ clean-room: لا نفكك برامج مغلقة مثل كلك أو Tasmeem. الأساس هو harfrust الموجود، وخطوط Google المفتوحة (Amiri وAref Ruqaa وReem Kufi وScheherazade New).
- **الاستقلال (hard fork):** حمزة يريد لاحقاً fork مستقلاً، مع نسب الأصل في قائمة منفصلة.
  - **إلزامي:** ملفات الرخصة، وسطور الحقوق، ومحتوى NOTICE.
  - **اختياري:** الشكر في README أو About، ولافتة "forked from" (فصلها عبر دعم GitHub).
  - **الخطة:** `CREDITS.md`، وسطر واحد في README وAbout.
  - **التوقيت:** بعد أن تتعمق تعديلاتنا (RTL، الخطّاط، الأسماء الداخلية).
  - البرامج المبنية على GPL تبقى GPL.
- الحزمة صارت 10 برامج. ترتيب الأولوية:
  1. rsm مع RTL والخطّاط.
  2. 7rrek.
  3. ttshat.
  4. mzika.
  5. الباقي.

## 5. شو بدنا نعمل (بالترتيب)

1. ~~إصلاح الاختبار ورفع الويب~~ ✅
2. دمج upstream بسكيل `shibshib-sync`.
3. **قلب الواجهة لليمين (RTL):** اللوحات والقوائم وشريط الأدوات والمحاذاة. هذا أصعب جزء، لأن egui لا يقلب الواجهة تلقائياً.
4. ~~إزالة بقايا ArtCraft من الواجهة~~ ✅
5. **الخطّاط:** لوحة الخط العربي (انظر §4.5).
6. اقتراح الترجمة العربية على upstream كـ Pull Request.
7. **7rrek:** timeline وkeyframes وonion skin وmotion/shape tweens وsymbols بـ timeline خاص، والتصدير إلى فيديو وGIF وLottie.
8. بعدها: ttshat، ثم mzika، ثم tlween وtsweer وeffectat، ثم aswat، ثم trteeb (عندما يجهز upstream)، ثم sharek. كلها عبر سكيل `shibshib-fork-app`.
9. لاحقاً: تغيير الأسماء الداخلية بالكامل، وربط الموقع the-247.com.

## 6. أوامر متكررة

```sh
cd ~/Documents/ShibShib/rsm
# مزامنة مع upstream
git fetch upstream && git merge upstream/main && python3 shibshib/rebrand.py
# الاختبارات
cargo test -q -p vectorcraft-ui-egui && cargo xtask assets && cargo xtask brands
# نسخة Mac
cargo xtask bundle && mv dist/VectorCraft.app "dist/ShibShib rsm.app"
# نسخة الويب (rustup مثبّت عبر brew وهو keg-only)
cd apps/vectorcraft-web && PATH="/opt/homebrew/opt/rustup/bin:$PATH" trunk build --release
# رفع الويب: انسخ dist/web إلى مجلد git على فرع gh-pages، وأضف .nojekyll، ثم push -f إلى gh-pages
# الترجمة
python3 shibshib/ar-work/check.py shibshib/ar-work NN   # فحص دفعة
python3 shibshib/ar-work/assemble.py                    # بناء ar.tsv
```

اختبار الواجهة في المتصفح: Playwright (`playwright-core` مع Google Chrome)، والـ locale على `ar` حتى يفتح البرنامج بالعربي.

## 7. الأدوار (نحن فريق)

| | حمزة (المصمم وصاحب القرار) | Claude (المهندس) |
|---|---|---|
| الرؤية | يحدد البرامج والأولويات والهوية | يقترح الطريق التقني الأبسط والأسلم |
| التصميم | اللوغوهات والألوان، وتجربة الاستخدام كمصمم محترف | يطبّق الهوية في الكود والأيقونات |
| الكود | — | يكتب ويختبر ويرفع، ويحافظ على الرخص والـ credits |
| الاختبار | يجرّب البرنامج فعلياً ويبلّغ عن المشاكل | اختبارات آلية وscreenshots قبل كل تسليم |
| الترجمة | يراجع المصطلحات العربية | يترجم على دفعات مع فحص آلي |
| القرارات الصعبة | حذف، أو نشر، أو تغيير اتجاه، أو تكاليف (سيرفر ودومين) | يسأل قبل أي خطوة لا رجعة فيها |

**التسلسل في كل مهمة:**
1. حمزة يطلب.
2. Claude يفحص ويقترح (بدون تنفيذ إذا طُلب ذلك).
3. حمزة يوافق.
4. Claude ينفّذ ويختبر.
5. Claude يرفع ويعطي رابطاً أو ملفاً.
6. حمزة يجرّب ويعطي ملاحظات.
7. Claude يحدّث هذا الملف.

## 8. السكيلز

**الموجودة ونستعملها:**

| السكيل | متى |
|---|---|
| `code-review` | قبل أي merge كبير، أو بعد ميزة جديدة (مستوى high) |
| `simplify` | بعد كل ميزة، لتنظيف الكود وإزالة التكرار |
| `security-review` | قبل أي إصدار، وخاصة لاحقاً مع sharek والسيرفر |
| `run` | لتشغيل البرنامج والتأكد من التعديل فعلياً |
| `replica-brand` | فحص بقايا اسم أو شعار ArtCraft بعد كل دمج |
| `replica-test` | خطة اختبار كاملة للأدوات (click-through وPlaywright) |
| `replica-diff` | قياس التكافؤ مع Illustrator وما الذي ينقص |
| `ponytail` | أبسط حل يعمل، ومنع التعقيد |
| `humanizer` | تحسين نصوص الـ README والإعلانات قبل النشر |
| `artifact-design` و`docs` | صفحات وتقارير نشاركها |
| `loop` / `schedule` | متابعة upstream بشكل دوري، مثلاً تقرير يومي بالجديد |
| `skill-creator` | لبناء السكيلز الخاصة بنا (تحت) |
| `fewer-permission-prompts` | تقليل طلبات الإذن للأوامر الآمنة المتكررة |

**سكيلزنا الخاصة (موجودة في `.claude/skills/` داخل الريبو، وتعمل عندما تبدأ الجلسة من مجلد الريبو):**
1. **`shibshib-sync`:** fetch، ثم merge من upstream، ثم `rebrand.py`، ثم الاختبارات، ثم assets وbrands، ثم تقرير.
2. **`shibshib-release`:** بناء Mac والويب، وscreenshots عربي وإنجليزي، ورفع gh-pages. فيه `scripts/release.sh` و`scripts/screenshot.mjs`.
3. **`shibshib-arabic`:** إضافة الجمل الجديدة للترجمة (`scripts/new_strings.py`)، مع قاموس مصطلحات ثابت.
4. **`shibshib-fork-app`:** وصفة fork برنامج جديد (تقييم، fork، rebrand، credits، عربي، نشر).

**قواعد الجودة:**
- لا commit بدون اختبارات تمرّ، أو ذكر صريح للاختبار الفاشل وسببه.
- تعديلاتنا على كود upstream تكون صغيرة ومعزولة، ويُفضّل أن تكون في ملفات جديدة. هذا يقلل تعارضات الدمج.
- كل تعديل يُسجَّل في `CHANGES.md`.
- screenshot قبل أي تسليم واجهة.
