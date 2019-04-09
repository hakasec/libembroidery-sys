#ifndef LIBEMBROIDERY_SYS_H
#define LIBEMBROIDERY_SYS_H

#include "libembroidery/api-start.h"
#include "libembroidery/api-stop.h"
#include "libembroidery/compound-file.h"
#include "libembroidery/compound-file-common.h"
#include "libembroidery/compound-file-difat.h"
#include "libembroidery/compound-file-directory.h"
#include "libembroidery/compound-file-fat.h"
#include "libembroidery/compound-file-header.h"
#include "libembroidery/emb-arc.h"
#include "libembroidery/emb-circle.h"
#include "libembroidery/emb-compress.h"
#include "libembroidery/emb-color.h"
#include "libembroidery/emb-ellipse.h"
#include "libembroidery/emb-file.h"
#include "libembroidery/emb-flag.h"
#include "libembroidery/emb-format.h"
#include "libembroidery/emb-hash.h"
#include "libembroidery/emb-hoop.h"
#include "libembroidery/emb-layer.h"
#include "libembroidery/emb-line.h"
#include "libembroidery/emb-logging.h"
#include "libembroidery/emb-path.h"
#include "libembroidery/emb-pattern.h"
#include "libembroidery/emb-point.h"
#include "libembroidery/emb-polygon.h"
#include "libembroidery/emb-polyline.h"
#include "libembroidery/emb-reader-writer.h"
#include "libembroidery/emb-rect.h"
#include "libembroidery/emb-satin-line.h"
#include "libembroidery/emb-settings.h"
#include "libembroidery/emb-spline.h"
#include "libembroidery/emb-stitch.h"
#include "libembroidery/emb-thread.h"
#include "libembroidery/emb-time.h"
#include "libembroidery/emb-vector.h"
#include "libembroidery/hashtable.h"
#include "libembroidery/helpers-binary.h"
#include "libembroidery/helpers-misc.h"
#include "libembroidery/helpers-unused.h"
#include "libembroidery/thread-color.h"
#include "libembroidery/formats.h"
#include "libembroidery/format-10o.h"
#include "libembroidery/format-100.h"
#include "libembroidery/format-art.h"
#include "libembroidery/format-bmc.h"
#include "libembroidery/format-bro.h"
#include "libembroidery/format-cnd.h"
#include "libembroidery/format-col.h"
#include "libembroidery/format-csd.h"
#include "libembroidery/format-csv.h"
#include "libembroidery/format-dat.h"
#include "libembroidery/format-dem.h"
#include "libembroidery/format-dsb.h"
#include "libembroidery/format-dst.h"
#include "libembroidery/format-dsz.h"
#include "libembroidery/format-dxf.h"
#include "libembroidery/format-edr.h"
#include "libembroidery/format-emd.h"
#include "libembroidery/format-exp.h"
#include "libembroidery/format-exy.h"
#include "libembroidery/format-eys.h"
#include "libembroidery/format-fxy.h"
#include "libembroidery/format-gc.h"
#include "libembroidery/format-gnc.h"
#include "libembroidery/format-gt.h"
#include "libembroidery/format-hus.h"
#include "libembroidery/format-inb.h"
#include "libembroidery/format-inf.h"
#include "libembroidery/format-jef.h"
#include "libembroidery/format-ksm.h"
#include "libembroidery/format-max.h"
#include "libembroidery/format-mit.h"
#include "libembroidery/format-new.h"
#include "libembroidery/format-ofm.h"
#include "libembroidery/format-pcd.h"
#include "libembroidery/format-pcm.h"
#include "libembroidery/format-pcq.h"
#include "libembroidery/format-pcs.h"
#include "libembroidery/format-pec.h"
#include "libembroidery/format-pel.h"
#include "libembroidery/format-pem.h"
#include "libembroidery/format-pes.h"
#include "libembroidery/format-phb.h"
#include "libembroidery/format-phc.h"
#include "libembroidery/format-plt.h"
#include "libembroidery/format-rgb.h"
#include "libembroidery/format-sew.h"
#include "libembroidery/format-shv.h"
#include "libembroidery/format-sst.h"
#include "libembroidery/format-stx.h"
#include "libembroidery/format-svg.h"
#include "libembroidery/format-t01.h"
#include "libembroidery/format-t09.h"
#include "libembroidery/format-tap.h"
#include "libembroidery/format-thr.h"
#include "libembroidery/format-txt.h"
#include "libembroidery/format-u00.h"
#include "libembroidery/format-u01.h"
#include "libembroidery/format-vip.h"
#include "libembroidery/format-vp3.h"
#include "libembroidery/format-xxx.h"
#include "libembroidery/format-zsk.h"

#endif

