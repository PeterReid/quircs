use libc;

use crate::quirc::*;
use crate::version_db::*;

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn rint(_: libc::c_double) -> libc::c_double;
}
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct neighbour_list {
    pub n: [neighbour; 32],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct neighbour {
    pub index: libc::c_int,
    pub distance: libc::c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon_score_data {
    pub ref_0: quirc_point,
    pub scores: [libc::c_int; 4],
    pub corners: *mut quirc_point,
}
pub type span_func_t =
    Option<unsafe fn(_: *mut libc::c_void, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> ()>;
/* Limits on the maximum size of QR-codes and their content. */
/* QR-code ECC types. */
/* QR-code data types. */
/* Common character encodings */
/* This structure is used to return information about detected QR codes
 * in the input image.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quirc_code {
    pub corners: [quirc_point; 4],
    pub size: libc::c_int,
    pub cell_bitmap: [uint8_t; 3917],
}
/* quirc - QR-code recognition library
 * Copyright (C) 2010-2012 Daniel Beer <dlbeer@gmail.com>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/* ***********************************************************************
 * Linear algebra routines
 */
unsafe fn line_intersect(
    mut p0: *const quirc_point,
    mut p1: *const quirc_point,
    mut q0: *const quirc_point,
    mut q1: *const quirc_point,
    mut r: *mut quirc_point,
) -> libc::c_int {
    /* (a, b) is perpendicular to line p */
    let mut a: libc::c_int = -((*p1).y - (*p0).y);
    let mut b: libc::c_int = (*p1).x - (*p0).x;
    /* (c, d) is perpendicular to line q */
    let mut c: libc::c_int = -((*q1).y - (*q0).y);
    let mut d: libc::c_int = (*q1).x - (*q0).x;
    /* e and f are dot products of the respective vectors with p and q */
    let mut e: libc::c_int = a * (*p1).x + b * (*p1).y;
    let mut f: libc::c_int = c * (*q1).x + d * (*q1).y;
    /* Now we need to solve:
     *     [a b] [rx]   [e]
     *     [c d] [ry] = [f]
     *
     * We do this by inverting the matrix and applying it to (e, f):
     *       [ d -b] [e]   [rx]
     * 1/det [-c  a] [f] = [ry]
     */
    let mut det: libc::c_int = a * d - b * c;
    if det == 0 {
        return 0i32;
    }
    (*r).x = (d * e - b * f) / det;
    (*r).y = (-c * e + a * f) / det;
    return 1i32;
}
unsafe fn perspective_setup(
    mut c: *mut libc::c_double,
    mut rect: *const quirc_point,
    mut w: libc::c_double,
    mut h: libc::c_double,
) {
    let mut x0: libc::c_double = (*rect.offset(0)).x as libc::c_double;
    let mut y0: libc::c_double = (*rect.offset(0)).y as libc::c_double;
    let mut x1: libc::c_double = (*rect.offset(1)).x as libc::c_double;
    let mut y1: libc::c_double = (*rect.offset(1)).y as libc::c_double;
    let mut x2: libc::c_double = (*rect.offset(2)).x as libc::c_double;
    let mut y2: libc::c_double = (*rect.offset(2)).y as libc::c_double;
    let mut x3: libc::c_double = (*rect.offset(3)).x as libc::c_double;
    let mut y3: libc::c_double = (*rect.offset(3)).y as libc::c_double;
    let mut wden: libc::c_double = w * (x2 * y3 - x3 * y2 + (x3 - x2) * y1 + x1 * (y2 - y3));
    let mut hden: libc::c_double = h * (x2 * y3 + x1 * (y2 - y3) - x3 * y2 + (x3 - x2) * y1);
    *c.offset(0) = (x1 * (x2 * y3 - x3 * y2)
        + x0 * (-x2 * y3 + x3 * y2 + (x2 - x3) * y1)
        + x1 * (x3 - x2) * y0)
        / wden;
    *c.offset(1) = -(x0 * (x2 * y3 + x1 * (y2 - y3) - x2 * y1) - x1 * x3 * y2
        + x2 * x3 * y1
        + (x1 * x3 - x2 * x3) * y0)
        / hden;
    *c.offset(2) = x0;
    *c.offset(3) = (y0 * (x1 * (y3 - y2) - x2 * y3 + x3 * y2)
        + y1 * (x2 * y3 - x3 * y2)
        + x0 * y1 * (y2 - y3))
        / wden;
    *c.offset(4) = (x0 * (y1 * y3 - y2 * y3) + x1 * y2 * y3 - x2 * y1 * y3
        + y0 * (x3 * y2 - x1 * y2 + (x2 - x3) * y1))
        / hden;
    *c.offset(5) = y0;
    *c.offset(6) = (x1 * (y3 - y2) + x0 * (y2 - y3) + (x2 - x3) * y1 + (x3 - x2) * y0) / wden;
    *c.offset(7) =
        (-x2 * y3 + x1 * y3 + x3 * y2 + x0 * (y1 - y2) - x3 * y1 + (x2 - x1) * y0) / hden;
}
unsafe fn perspective_map(
    mut c: *const libc::c_double,
    mut u: libc::c_double,
    mut v: libc::c_double,
    mut ret: *mut quirc_point,
) {
    let mut den: libc::c_double = *c.offset(6) * u + *c.offset(7) * v + 1.0f64;
    let mut x: libc::c_double = (*c.offset(0) * u + *c.offset(1) * v + *c.offset(2)) / den;
    let mut y: libc::c_double = (*c.offset(3) * u + *c.offset(4) * v + *c.offset(5)) / den;
    (*ret).x = rint(x) as libc::c_int;
    (*ret).y = rint(y) as libc::c_int;
}
unsafe fn perspective_unmap(
    mut c: *const libc::c_double,
    mut in_0: *const quirc_point,
    mut u: *mut libc::c_double,
    mut v: *mut libc::c_double,
) {
    let mut x: libc::c_double = (*in_0).x as libc::c_double;
    let mut y: libc::c_double = (*in_0).y as libc::c_double;
    let mut den: libc::c_double = -*c.offset(0) * *c.offset(7) * y
        + *c.offset(1) * *c.offset(6) * y
        + (*c.offset(3) * *c.offset(7) - *c.offset(4) * *c.offset(6)) * x
        + *c.offset(0) * *c.offset(4)
        - *c.offset(1) * *c.offset(3);
    *u = -(*c.offset(1) * (y - *c.offset(5)) - *c.offset(2) * *c.offset(7) * y
        + (*c.offset(5) * *c.offset(7) - *c.offset(4)) * x
        + *c.offset(2) * *c.offset(4))
        / den;
    *v = (*c.offset(0) * (y - *c.offset(5)) - *c.offset(2) * *c.offset(6) * y
        + (*c.offset(5) * *c.offset(6) - *c.offset(3)) * x
        + *c.offset(2) * *c.offset(3))
        / den;
}
unsafe fn flood_fill_seed(
    mut q: *mut Quirc,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut func: span_func_t,
    mut user_data: *mut libc::c_void,
    mut depth: libc::c_int,
) {
    let mut left: libc::c_int = x;
    let mut right: libc::c_int = x;
    let mut i: libc::c_int = 0;
    let mut row: *mut quirc_pixel_t = (*q).pixels.offset((y * (*q).w) as isize);
    if depth >= 4096i32 {
        return;
    }
    while left > 0i32 && *row.offset((left - 1i32) as isize) as libc::c_int == from {
        left -= 1
    }
    while right < (*q).w - 1i32 && *row.offset((right + 1i32) as isize) as libc::c_int == from {
        right += 1
    }
    /* Fill the extent */
    i = left;
    while i <= right {
        *row.offset(i as isize) = to as quirc_pixel_t;
        i += 1
    }
    if func.is_some() {
        func.expect("non-null function pointer")(user_data, y, left, right);
    }
    /* Seed new flood-fills */
    if y > 0i32 {
        row = (*q).pixels.offset(((y - 1i32) * (*q).w) as isize);
        i = left;
        while i <= right {
            if *row.offset(i as isize) as libc::c_int == from {
                flood_fill_seed(q, i, y - 1i32, from, to, func, user_data, depth + 1i32);
            }
            i += 1
        }
    }
    if y < (*q).h - 1i32 {
        row = (*q).pixels.offset(((y + 1i32) * (*q).w) as isize);
        i = left;
        while i <= right {
            if *row.offset(i as isize) as libc::c_int == from {
                flood_fill_seed(q, i, y + 1i32, from, to, func, user_data, depth + 1i32);
            }
            i += 1
        }
    };
}
/* ***********************************************************************
 * Adaptive thresholding
 */
unsafe fn otsu(mut q: *const Quirc) -> uint8_t {
    let mut numPixels: libc::c_int = (*q).w * (*q).h;
    // Calculate histogram
    let mut histogram: [libc::c_uint; 256] = [0; 256];
    memset(
        histogram.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_uint; 256]>() as libc::c_ulong,
    );
    let mut ptr: *mut uint8_t = (*q).image;
    let mut length: libc::c_int = numPixels;
    loop {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        let mut value: uint8_t = *fresh1;
        histogram[value as usize] = histogram[value as usize].wrapping_add(1)
    }
    // Calculate weighted sum of histogram values
    let mut sum: libc::c_uint = 0i32 as libc::c_uint;
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    i = 0i32 as libc::c_uint;
    while i <= 255i32 as libc::c_uint {
        sum = sum.wrapping_add(i.wrapping_mul(histogram[i as usize]));
        i = i.wrapping_add(1)
    }
    // Compute threshold
    let mut sumB: libc::c_int = 0i32;
    let mut q1: libc::c_int = 0i32;
    let mut max: libc::c_double = 0i32 as libc::c_double;
    let mut threshold: uint8_t = 0i32 as uint8_t;
    i = 0i32 as libc::c_uint;
    while i <= 255i32 as libc::c_uint {
        // Weighted background
        q1 = (q1 as libc::c_uint).wrapping_add(histogram[i as usize]) as libc::c_int as libc::c_int;
        if !(q1 == 0i32) {
            // Weighted foreground
            let q2: libc::c_int = numPixels - q1;
            if q2 == 0i32 {
                break;
            }
            sumB = (sumB as libc::c_uint).wrapping_add(i.wrapping_mul(histogram[i as usize]))
                as libc::c_int as libc::c_int;
            let m1: libc::c_double = sumB as libc::c_double / q1 as libc::c_double;
            let m2: libc::c_double =
                (sum as libc::c_double - sumB as libc::c_double) / q2 as libc::c_double;
            let m1m2: libc::c_double = m1 - m2;
            let variance: libc::c_double =
                m1m2 * m1m2 * q1 as libc::c_double * q2 as libc::c_double;
            if variance >= max {
                threshold = i as uint8_t;
                max = variance
            }
        }
        i = i.wrapping_add(1)
    }
    return threshold;
}
unsafe fn area_count(
    mut user_data: *mut libc::c_void,
    mut y: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    (*(user_data as *mut quirc_region)).count += right - left + 1i32;
}
unsafe fn region_code(mut q: *mut Quirc, mut x: libc::c_int, mut y: libc::c_int) -> libc::c_int {
    let mut pixel: libc::c_int = 0;
    let mut box_0: *mut quirc_region = 0 as *mut quirc_region;
    let mut region: libc::c_int = 0;
    if x < 0i32 || y < 0i32 || x >= (*q).w || y >= (*q).h {
        return -1i32;
    }
    pixel = *(*q).pixels.offset((y * (*q).w + x) as isize) as libc::c_int;
    if pixel >= 2i32 {
        return pixel;
    }
    if pixel == 0i32 {
        return -1i32;
    }
    if (*q).num_regions >= 65534i32 {
        return -1i32;
    }
    region = (*q).num_regions;
    let fresh2 = (*q).num_regions;
    (*q).num_regions = (*q).num_regions + 1;
    box_0 = &mut *(*q).regions.as_mut_ptr().offset(fresh2 as isize) as *mut quirc_region;
    memset(
        box_0 as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<quirc_region>() as libc::c_ulong,
    );
    (*box_0).seed.x = x;
    (*box_0).seed.y = y;
    (*box_0).capstone = -1i32;
    flood_fill_seed(
        q,
        x,
        y,
        pixel,
        region,
        Some(
            area_count
                as unsafe fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        box_0 as *mut libc::c_void,
        0i32,
    );
    return region;
}
unsafe fn find_one_corner(
    mut user_data: *mut libc::c_void,
    mut y: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    let mut psd: *mut polygon_score_data = user_data as *mut polygon_score_data;
    let mut xs: [libc::c_int; 2] = [left, right];
    let mut dy: libc::c_int = y - (*psd).ref_0.y;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 2i32 {
        let mut dx: libc::c_int = xs[i as usize] - (*psd).ref_0.x;
        let mut d: libc::c_int = dx * dx + dy * dy;
        if d > (*psd).scores[0] {
            (*psd).scores[0] = d;
            (*(*psd).corners.offset(0)).x = xs[i as usize];
            (*(*psd).corners.offset(0)).y = y
        }
        i += 1
    }
}
unsafe fn find_other_corners(
    mut user_data: *mut libc::c_void,
    mut y: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    let mut psd: *mut polygon_score_data = user_data as *mut polygon_score_data;
    let mut xs: [libc::c_int; 2] = [left, right];
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 2i32 {
        let mut up: libc::c_int = xs[i as usize] * (*psd).ref_0.x + y * (*psd).ref_0.y;
        let mut right_0: libc::c_int = xs[i as usize] * -(*psd).ref_0.y + y * (*psd).ref_0.x;
        let mut scores: [libc::c_int; 4] = [up, right_0, -up, -right_0];
        let mut j: libc::c_int = 0;
        j = 0i32;
        while j < 4i32 {
            if scores[j as usize] > (*psd).scores[j as usize] {
                (*psd).scores[j as usize] = scores[j as usize];
                (*(*psd).corners.offset(j as isize)).x = xs[i as usize];
                (*(*psd).corners.offset(j as isize)).y = y
            }
            j += 1
        }
        i += 1
    }
}
unsafe fn find_region_corners(
    mut q: *mut Quirc,
    mut rcode: libc::c_int,
    mut ref_0: *const quirc_point,
    mut corners: *mut quirc_point,
) {
    let mut region: *mut quirc_region =
        &mut *(*q).regions.as_mut_ptr().offset(rcode as isize) as *mut quirc_region;
    let mut psd: polygon_score_data = polygon_score_data {
        ref_0: quirc_point { x: 0, y: 0 },
        scores: [0; 4],
        corners: 0 as *mut quirc_point,
    };
    let mut i: libc::c_int = 0;
    memset(
        &mut psd as *mut polygon_score_data as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<polygon_score_data>() as libc::c_ulong,
    );
    psd.corners = corners;
    memcpy(
        &mut psd.ref_0 as *mut quirc_point as *mut libc::c_void,
        ref_0 as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    psd.scores[0] = -1i32;
    flood_fill_seed(
        q,
        (*region).seed.x,
        (*region).seed.y,
        rcode,
        1i32,
        Some(
            find_one_corner
                as unsafe fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        &mut psd as *mut polygon_score_data as *mut libc::c_void,
        0i32,
    );
    psd.ref_0.x = (*psd.corners.offset(0)).x - psd.ref_0.x;
    psd.ref_0.y = (*psd.corners.offset(0)).y - psd.ref_0.y;
    i = 0i32;
    while i < 4i32 {
        memcpy(
            &mut *psd.corners.offset(i as isize) as *mut quirc_point as *mut libc::c_void,
            &mut (*region).seed as *mut quirc_point as *const libc::c_void,
            ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
        );
        i += 1
    }
    i = (*region).seed.x * psd.ref_0.x + (*region).seed.y * psd.ref_0.y;
    psd.scores[0] = i;
    psd.scores[2] = -i;
    i = (*region).seed.x * -psd.ref_0.y + (*region).seed.y * psd.ref_0.x;
    psd.scores[1] = i;
    psd.scores[3] = -i;
    flood_fill_seed(
        q,
        (*region).seed.x,
        (*region).seed.y,
        1i32,
        rcode,
        Some(
            find_other_corners
                as unsafe fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        &mut psd as *mut polygon_score_data as *mut libc::c_void,
        0i32,
    );
}
unsafe fn record_capstone(mut q: *mut Quirc, mut ring: libc::c_int, mut stone: libc::c_int) {
    let mut stone_reg: *mut quirc_region =
        &mut *(*q).regions.as_mut_ptr().offset(stone as isize) as *mut quirc_region;
    let mut ring_reg: *mut quirc_region =
        &mut *(*q).regions.as_mut_ptr().offset(ring as isize) as *mut quirc_region;
    let mut capstone: *mut quirc_capstone = 0 as *mut quirc_capstone;
    let mut cs_index: libc::c_int = 0;
    if (*q).num_capstones >= 32i32 {
        return;
    }
    cs_index = (*q).num_capstones;
    let fresh3 = (*q).num_capstones;
    (*q).num_capstones = (*q).num_capstones + 1;
    capstone = &mut *(*q).capstones.as_mut_ptr().offset(fresh3 as isize) as *mut quirc_capstone;
    memset(
        capstone as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<quirc_capstone>() as libc::c_ulong,
    );
    (*capstone).qr_grid = -1i32;
    (*capstone).ring = ring;
    (*capstone).stone = stone;
    (*stone_reg).capstone = cs_index;
    (*ring_reg).capstone = cs_index;
    /* Find the corners of the ring */
    find_region_corners(
        q,
        ring,
        &mut (*stone_reg).seed,
        (*capstone).corners.as_mut_ptr(),
    );
    /* Set up the perspective transform and find the center */
    perspective_setup(
        (*capstone).c.as_mut_ptr(),
        (*capstone).corners.as_mut_ptr(),
        7.0f64,
        7.0f64,
    );
    perspective_map(
        (*capstone).c.as_mut_ptr(),
        3.5f64,
        3.5f64,
        &mut (*capstone).center,
    );
}
unsafe fn test_capstone(
    mut q: *mut Quirc,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut pb: *mut libc::c_int,
) {
    let mut ring_right: libc::c_int = region_code(q, x - *pb.offset(4), y);
    let mut stone: libc::c_int =
        region_code(q, x - *pb.offset(4) - *pb.offset(3) - *pb.offset(2), y);
    let mut ring_left: libc::c_int = region_code(
        q,
        x - *pb.offset(4) - *pb.offset(3) - *pb.offset(2) - *pb.offset(1) - *pb.offset(0),
        y,
    );
    let mut stone_reg: *mut quirc_region = 0 as *mut quirc_region;
    let mut ring_reg: *mut quirc_region = 0 as *mut quirc_region;
    let mut ratio: libc::c_int = 0;
    if ring_left < 0i32 || ring_right < 0i32 || stone < 0i32 {
        return;
    }
    /* Left and ring of ring should be connected */
    if ring_left != ring_right {
        return;
    }
    /* Ring should be disconnected from stone */
    if ring_left == stone {
        return;
    }
    stone_reg = &mut *(*q).regions.as_mut_ptr().offset(stone as isize) as *mut quirc_region;
    ring_reg = &mut *(*q).regions.as_mut_ptr().offset(ring_left as isize) as *mut quirc_region;
    /* Already detected */
    if (*stone_reg).capstone >= 0i32 || (*ring_reg).capstone >= 0i32 {
        return;
    }
    /* Ratio should ideally be 37.5 */
    ratio = (*stone_reg).count * 100i32 / (*ring_reg).count;
    if ratio < 10i32 || ratio > 70i32 {
        return;
    }
    record_capstone(q, ring_left, stone);
}
unsafe fn finder_scan(mut q: *mut Quirc, mut y: libc::c_int) {
    let mut row: *mut quirc_pixel_t = (*q).pixels.offset((y * (*q).w) as isize);
    let mut x: libc::c_int = 0;
    let mut last_color: libc::c_int = 0i32;
    let mut run_length: libc::c_int = 0i32;
    let mut run_count: libc::c_int = 0i32;
    let mut pb: [libc::c_int; 5] = [0; 5];
    memset(
        pb.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong,
    );
    x = 0i32;
    while x < (*q).w {
        let mut color: libc::c_int = if *row.offset(x as isize) as libc::c_int != 0 {
            1i32
        } else {
            0i32
        };
        if x != 0 && color != last_color {
            memmove(
                pb.as_mut_ptr() as *mut libc::c_void,
                pb.as_mut_ptr().offset(1) as *const libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(4i32 as libc::c_ulong),
            );
            pb[4] = run_length;
            run_length = 0i32;
            run_count += 1;
            if color == 0 && run_count >= 5i32 {
                static mut check: [libc::c_int; 5] = [1i32, 1i32, 3i32, 1i32, 1i32];
                let mut avg: libc::c_int = 0;
                let mut err: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                let mut ok: libc::c_int = 1i32;
                avg = (pb[0] + pb[1] + pb[3] + pb[4]) / 4i32;
                err = avg * 3i32 / 4i32;
                i = 0i32;
                while i < 5i32 {
                    if pb[i as usize] < check[i as usize] * avg - err
                        || pb[i as usize] > check[i as usize] * avg + err
                    {
                        ok = 0i32
                    }
                    i += 1
                }
                if ok != 0 {
                    test_capstone(q, x, y, pb.as_mut_ptr());
                }
            }
        }
        run_length += 1;
        last_color = color;
        x += 1
    }
}
unsafe fn find_alignment_pattern(mut q: *mut Quirc, mut index: libc::c_int) {
    let mut qr: *mut quirc_grid =
        &mut *(*q).grids.as_mut_ptr().offset(index as isize) as *mut quirc_grid;
    let mut c0: *mut quirc_capstone = &mut *(*q)
        .capstones
        .as_mut_ptr()
        .offset(*(*qr).caps.as_mut_ptr().offset(0) as isize)
        as *mut quirc_capstone;
    let mut c2: *mut quirc_capstone = &mut *(*q)
        .capstones
        .as_mut_ptr()
        .offset(*(*qr).caps.as_mut_ptr().offset(2) as isize)
        as *mut quirc_capstone;
    let mut a: quirc_point = quirc_point { x: 0, y: 0 };
    let mut b: quirc_point = quirc_point { x: 0, y: 0 };
    let mut c: quirc_point = quirc_point { x: 0, y: 0 };
    let mut size_estimate: libc::c_int = 0;
    let mut step_size: libc::c_int = 1i32;
    let mut dir: libc::c_int = 0i32;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    /* Grab our previous estimate of the alignment pattern corner */
    memcpy(
        &mut b as *mut quirc_point as *mut libc::c_void,
        &mut (*qr).align as *mut quirc_point as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    /* Guess another two corners of the alignment pattern so that we
     * can estimate its size.
     */
    perspective_unmap((*c0).c.as_mut_ptr(), &mut b, &mut u, &mut v);
    perspective_map((*c0).c.as_mut_ptr(), u, v + 1.0f64, &mut a);
    perspective_unmap((*c2).c.as_mut_ptr(), &mut b, &mut u, &mut v);
    perspective_map((*c2).c.as_mut_ptr(), u + 1.0f64, v, &mut c);
    size_estimate = abs((a.x - b.x) * -(c.y - b.y) + (a.y - b.y) * (c.x - b.x));
    /* Spiral outwards from the estimate point until we find something
     * roughly the right size. Don't look too far from the estimate
     * point.
     */
    while step_size * step_size < size_estimate * 100i32 {
        static mut dx_map: [libc::c_int; 4] = [1i32, 0i32, -1i32, 0i32];
        static mut dy_map: [libc::c_int; 4] = [0i32, -1i32, 0i32, 1i32];
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < step_size {
            let mut code: libc::c_int = region_code(q, b.x, b.y);
            if code >= 0i32 {
                let mut reg: *mut quirc_region =
                    &mut *(*q).regions.as_mut_ptr().offset(code as isize) as *mut quirc_region;
                if (*reg).count >= size_estimate / 2i32 && (*reg).count <= size_estimate * 2i32 {
                    (*qr).align_region = code;
                    return;
                }
            }
            b.x += dx_map[dir as usize];
            b.y += dy_map[dir as usize];
            i += 1
        }
        dir = (dir + 1i32) % 4i32;
        if dir & 1i32 == 0 {
            step_size += 1
        }
    }
}
unsafe fn find_leftmost_to_line(
    mut user_data: *mut libc::c_void,
    mut y: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    let mut psd: *mut polygon_score_data = user_data as *mut polygon_score_data;
    let mut xs: [libc::c_int; 2] = [left, right];
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 2i32 {
        let mut d: libc::c_int = -(*psd).ref_0.y * xs[i as usize] + (*psd).ref_0.x * y;
        if d < (*psd).scores[0] {
            (*psd).scores[0] = d;
            (*(*psd).corners.offset(0)).x = xs[i as usize];
            (*(*psd).corners.offset(0)).y = y
        }
        i += 1
    }
}
/* Do a Bresenham scan from one point to another and count the number
 * of black/white transitions.
 */
unsafe fn timing_scan(
    mut q: *const Quirc,
    mut p0: *const quirc_point,
    mut p1: *const quirc_point,
) -> libc::c_int {
    let mut n: libc::c_int = (*p1).x - (*p0).x;
    let mut d: libc::c_int = (*p1).y - (*p0).y;
    let mut x: libc::c_int = (*p0).x;
    let mut y: libc::c_int = (*p0).y;
    let mut dom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nondom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dom_step: libc::c_int = 0;
    let mut nondom_step: libc::c_int = 0;
    let mut a: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut run_length: libc::c_int = 0i32;
    let mut count: libc::c_int = 0i32;
    if (*p0).x < 0i32 || (*p0).y < 0i32 || (*p0).x >= (*q).w || (*p0).y >= (*q).h {
        return -1i32;
    }
    if (*p1).x < 0i32 || (*p1).y < 0i32 || (*p1).x >= (*q).w || (*p1).y >= (*q).h {
        return -1i32;
    }
    if abs(n) > abs(d) {
        let mut swap: libc::c_int = n;
        n = d;
        d = swap;
        dom = &mut x;
        nondom = &mut y
    } else {
        dom = &mut y;
        nondom = &mut x
    }
    if n < 0i32 {
        n = -n;
        nondom_step = -1i32
    } else {
        nondom_step = 1i32
    }
    if d < 0i32 {
        d = -d;
        dom_step = -1i32
    } else {
        dom_step = 1i32
    }
    x = (*p0).x;
    y = (*p0).y;
    i = 0i32;
    while i <= d {
        let mut pixel: libc::c_int = 0;
        if y < 0i32 || y >= (*q).h || x < 0i32 || x >= (*q).w {
            break;
        }
        pixel = *(*q).pixels.offset((y * (*q).w + x) as isize) as libc::c_int;
        if pixel != 0 {
            if run_length >= 2i32 {
                count += 1
            }
            run_length = 0i32
        } else {
            run_length += 1
        }
        a += n;
        *dom += dom_step;
        if a >= d {
            *nondom += nondom_step;
            a -= d
        }
        i += 1
    }
    return count;
}
/* Try the measure the timing pattern for a given QR code. This does
 * not require the global perspective to have been set up, but it
 * does require that the capstone corners have been set to their
 * canonical rotation.
 *
 * For each capstone, we find a point in the middle of the ring band
 * which is nearest the centre of the code. Using these points, we do
 * a horizontal and a vertical timing scan.
 */
unsafe fn measure_timing_pattern(mut q: *mut Quirc, mut index: libc::c_int) -> libc::c_int {
    let mut qr: *mut quirc_grid =
        &mut *(*q).grids.as_mut_ptr().offset(index as isize) as *mut quirc_grid;
    let mut i: libc::c_int = 0;
    let mut scan: libc::c_int = 0;
    let mut ver: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        static mut us: [libc::c_double; 3] = [6.5f64, 6.5f64, 0.5f64];
        static mut vs: [libc::c_double; 3] = [0.5f64, 6.5f64, 6.5f64];
        let mut cap: *mut quirc_capstone = &mut *(*q)
            .capstones
            .as_mut_ptr()
            .offset(*(*qr).caps.as_mut_ptr().offset(i as isize) as isize)
            as *mut quirc_capstone;
        perspective_map(
            (*cap).c.as_mut_ptr(),
            us[i as usize],
            vs[i as usize],
            &mut *(*qr).tpep.as_mut_ptr().offset(i as isize),
        );
        i += 1
    }
    (*qr).hscan = timing_scan(
        q,
        &mut *(*qr).tpep.as_mut_ptr().offset(1),
        &mut *(*qr).tpep.as_mut_ptr().offset(2),
    );
    (*qr).vscan = timing_scan(
        q,
        &mut *(*qr).tpep.as_mut_ptr().offset(1),
        &mut *(*qr).tpep.as_mut_ptr().offset(0),
    );
    scan = (*qr).hscan;
    if (*qr).vscan > scan {
        scan = (*qr).vscan
    }
    /* If neither scan worked, we can't go any further. */
    if scan < 0i32 {
        return -1i32;
    }
    /* Choose the nearest allowable grid size */
    size = scan * 2i32 + 13i32;
    ver = (size - 15i32) / 4i32;
    (*qr).grid_size = ver * 4i32 + 17i32;
    return 0i32;
}
/* Read a cell from a grid using the currently set perspective
 * transform. Returns +/- 1 for black/white, 0 for cells which are
 * out of image bounds.
 */
unsafe fn read_cell(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut qr: *const quirc_grid =
        &*(*q).grids.as_ptr().offset(index as isize) as *const quirc_grid;
    let mut p: quirc_point = quirc_point { x: 0, y: 0 };
    perspective_map(
        (*qr).c.as_ptr(),
        x as libc::c_double + 0.5f64,
        y as libc::c_double + 0.5f64,
        &mut p,
    );
    if p.y < 0i32 || p.y >= (*q).h || p.x < 0i32 || p.x >= (*q).w {
        return 0i32;
    }
    return if *(*q).pixels.offset((p.y * (*q).w + p.x) as isize) as libc::c_int != 0 {
        1i32
    } else {
        -1i32
    };
}
unsafe fn fitness_cell(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut qr: *const quirc_grid =
        &*(*q).grids.as_ptr().offset(index as isize) as *const quirc_grid;
    let mut score: libc::c_int = 0i32;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    v = 0i32;
    while v < 3i32 {
        u = 0i32;
        while u < 3i32 {
            static mut offsets: [libc::c_double; 3] = [0.3f64, 0.5f64, 0.7f64];
            let mut p: quirc_point = quirc_point { x: 0, y: 0 };
            perspective_map(
                (*qr).c.as_ptr(),
                x as libc::c_double + offsets[u as usize],
                y as libc::c_double + offsets[v as usize],
                &mut p,
            );
            if !(p.y < 0i32 || p.y >= (*q).h || p.x < 0i32 || p.x >= (*q).w) {
                if *(*q).pixels.offset((p.y * (*q).w + p.x) as isize) != 0 {
                    score += 1
                } else {
                    score -= 1
                }
            }
            u += 1
        }
        v += 1
    }
    return score;
}
unsafe fn fitness_ring(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut radius: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0i32;
    i = 0i32;
    while i < radius * 2i32 {
        score += fitness_cell(q, index, cx - radius + i, cy - radius);
        score += fitness_cell(q, index, cx - radius, cy + radius - i);
        score += fitness_cell(q, index, cx + radius, cy - radius + i);
        score += fitness_cell(q, index, cx + radius - i, cy + radius);
        i += 1
    }
    return score;
}
unsafe fn fitness_apat(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
) -> libc::c_int {
    return fitness_cell(q, index, cx, cy) - fitness_ring(q, index, cx, cy, 1i32)
        + fitness_ring(q, index, cx, cy, 2i32);
}
unsafe fn fitness_capstone(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    x += 3i32;
    y += 3i32;
    return fitness_cell(q, index, x, y) + fitness_ring(q, index, x, y, 1i32)
        - fitness_ring(q, index, x, y, 2i32)
        + fitness_ring(q, index, x, y, 3i32);
}
/* Compute a fitness score for the currently configured perspective
 * transform, using the features we expect to find by scanning the
 * grid.
 */
unsafe fn fitness_all(mut q: *const Quirc, mut index: libc::c_int) -> libc::c_int {
    let mut qr: *const quirc_grid =
        &*(*q).grids.as_ptr().offset(index as isize) as *const quirc_grid;
    let mut version: libc::c_int = ((*qr).grid_size - 17i32) / 4i32;
    let mut info: *const quirc_version_info =
        &*quirc_version_db.as_ptr().offset(version as isize) as *const quirc_version_info;
    let mut score: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ap_count: libc::c_int = 0;
    /* Check the timing pattern */
    i = 0i32;
    while i < (*qr).grid_size - 14i32 {
        let mut expect: libc::c_int = if i & 1i32 != 0 { 1i32 } else { -1i32 };
        score += fitness_cell(q, index, i + 7i32, 6i32) * expect;
        score += fitness_cell(q, index, 6i32, i + 7i32) * expect;
        i += 1
    }
    /* Check capstones */
    score += fitness_capstone(q, index, 0i32, 0i32);
    score += fitness_capstone(q, index, (*qr).grid_size - 7i32, 0i32);
    score += fitness_capstone(q, index, 0i32, (*qr).grid_size - 7i32);
    if version < 0i32 || version > 40i32 {
        return score;
    }
    /* Check alignment patterns */
    ap_count = 0i32;
    while ap_count < 7i32 && (*info).apat[ap_count as usize] != 0 {
        ap_count += 1
    }
    i = 1i32;
    while i + 1i32 < ap_count {
        score += fitness_apat(q, index, 6i32, (*info).apat[i as usize]);
        score += fitness_apat(q, index, (*info).apat[i as usize], 6i32);
        i += 1
    }
    i = 1i32;
    while i < ap_count {
        j = 1i32;
        while j < ap_count {
            score += fitness_apat(q, index, (*info).apat[i as usize], (*info).apat[j as usize]);
            j += 1
        }
        i += 1
    }
    return score;
}
unsafe fn jiggle_perspective(mut q: *mut Quirc, mut index: libc::c_int) {
    let mut qr: *mut quirc_grid =
        &mut *(*q).grids.as_mut_ptr().offset(index as isize) as *mut quirc_grid;
    let mut best: libc::c_int = fitness_all(q, index);
    let mut pass: libc::c_int = 0;
    let mut adjustments: [libc::c_double; 8] = [0.; 8];
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 8i32 {
        adjustments[i as usize] = (*qr).c[i as usize] * 0.02f64;
        i += 1
    }
    pass = 0i32;
    while pass < 5i32 {
        i = 0i32;
        while i < 16i32 {
            let mut j: libc::c_int = i >> 1i32;
            let mut test: libc::c_int = 0;
            let mut old: libc::c_double = (*qr).c[j as usize];
            let mut step: libc::c_double = adjustments[j as usize];
            let mut new: libc::c_double = 0.;
            if i & 1i32 != 0 {
                new = old + step
            } else {
                new = old - step
            }
            (*qr).c[j as usize] = new;
            test = fitness_all(q, index);
            if test > best {
                best = test
            } else {
                (*qr).c[j as usize] = old
            }
            i += 1
        }
        i = 0i32;
        while i < 8i32 {
            adjustments[i as usize] *= 0.5f64;
            i += 1
        }
        pass += 1
    }
}
/* Once the capstones are in place and an alignment point has been
 * chosen, we call this function to set up a grid-reading perspective
 * transform.
 */
unsafe fn setup_qr_perspective(mut q: *mut Quirc, mut index: libc::c_int) {
    let mut qr: *mut quirc_grid =
        &mut *(*q).grids.as_mut_ptr().offset(index as isize) as *mut quirc_grid;
    let mut rect: [quirc_point; 4] = [quirc_point { x: 0, y: 0 }; 4];
    /* Set up the perspective map for reading the grid */
    memcpy(
        &mut *rect.as_mut_ptr().offset(0) as *mut quirc_point as *mut libc::c_void,
        &mut *(*(*q)
            .capstones
            .as_mut_ptr()
            .offset(*(*qr).caps.as_mut_ptr().offset(1) as isize))
        .corners
        .as_mut_ptr()
        .offset(0) as *mut quirc_point as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    memcpy(
        &mut *rect.as_mut_ptr().offset(1) as *mut quirc_point as *mut libc::c_void,
        &mut *(*(*q)
            .capstones
            .as_mut_ptr()
            .offset(*(*qr).caps.as_mut_ptr().offset(2) as isize))
        .corners
        .as_mut_ptr()
        .offset(0) as *mut quirc_point as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    memcpy(
        &mut *rect.as_mut_ptr().offset(2) as *mut quirc_point as *mut libc::c_void,
        &mut (*qr).align as *mut quirc_point as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    memcpy(
        &mut *rect.as_mut_ptr().offset(3) as *mut quirc_point as *mut libc::c_void,
        &mut *(*(*q)
            .capstones
            .as_mut_ptr()
            .offset(*(*qr).caps.as_mut_ptr().offset(0) as isize))
        .corners
        .as_mut_ptr()
        .offset(0) as *mut quirc_point as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    perspective_setup(
        (*qr).c.as_mut_ptr(),
        rect.as_mut_ptr(),
        ((*qr).grid_size - 7i32) as libc::c_double,
        ((*qr).grid_size - 7i32) as libc::c_double,
    );
    jiggle_perspective(q, index);
}
/* Rotate the capstone with so that corner 0 is the leftmost with respect
 * to the given reference line.
 */
unsafe fn rotate_capstone(
    mut cap: *mut quirc_capstone,
    mut h0: *const quirc_point,
    mut hd: *const quirc_point,
) {
    let mut copy: [quirc_point; 4] = [quirc_point { x: 0, y: 0 }; 4];
    let mut j: libc::c_int = 0;
    let mut best: libc::c_int = 0i32;
    let mut best_score: libc::c_int = 2147483647i32;
    j = 0i32;
    while j < 4i32 {
        let mut p: *mut quirc_point =
            &mut *(*cap).corners.as_mut_ptr().offset(j as isize) as *mut quirc_point;
        let mut score: libc::c_int = ((*p).x - (*h0).x) * -(*hd).y + ((*p).y - (*h0).y) * (*hd).x;
        if j == 0 || score < best_score {
            best = j;
            best_score = score
        }
        j += 1
    }
    /* Rotate the capstone */
    j = 0i32;
    while j < 4i32 {
        memcpy(
            &mut *copy.as_mut_ptr().offset(j as isize) as *mut quirc_point as *mut libc::c_void,
            &mut *(*cap)
                .corners
                .as_mut_ptr()
                .offset(((j + best) % 4i32) as isize) as *mut quirc_point
                as *const libc::c_void,
            ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
        );
        j += 1
    }
    memcpy(
        (*cap).corners.as_mut_ptr() as *mut libc::c_void,
        copy.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[quirc_point; 4]>() as libc::c_ulong,
    );
    perspective_setup(
        (*cap).c.as_mut_ptr(),
        (*cap).corners.as_mut_ptr(),
        7.0f64,
        7.0f64,
    );
}
unsafe fn record_qr_grid(
    mut q: *mut Quirc,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) {
    let mut h0: quirc_point = quirc_point { x: 0, y: 0 };
    let mut hd: quirc_point = quirc_point { x: 0, y: 0 };
    let mut i: libc::c_int = 0;
    let mut qr_index: libc::c_int = 0;
    let mut qr: *mut quirc_grid = 0 as *mut quirc_grid;
    if (*q).num_grids >= 8i32 {
        return;
    }
    /* Construct the hypotenuse line from A to C. B should be to
     * the left of this line.
     */
    memcpy(
        &mut h0 as *mut quirc_point as *mut libc::c_void,
        &mut (*(*q).capstones.as_mut_ptr().offset(a as isize)).center as *mut quirc_point
            as *const libc::c_void,
        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
    );
    hd.x = (*q).capstones[c as usize].center.x - (*q).capstones[a as usize].center.x;
    hd.y = (*q).capstones[c as usize].center.y - (*q).capstones[a as usize].center.y;
    /* Make sure A-B-C is clockwise */
    if ((*q).capstones[b as usize].center.x - h0.x) * -hd.y
        + ((*q).capstones[b as usize].center.y - h0.y) * hd.x
        > 0i32
    {
        let mut swap: libc::c_int = a;
        a = c;
        c = swap;
        hd.x = -hd.x;
        hd.y = -hd.y
    }
    /* Record the grid and its components */
    qr_index = (*q).num_grids;
    let fresh4 = (*q).num_grids;
    (*q).num_grids = (*q).num_grids + 1;
    qr = &mut *(*q).grids.as_mut_ptr().offset(fresh4 as isize) as *mut quirc_grid;
    memset(
        qr as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<quirc_grid>() as libc::c_ulong,
    );
    (*qr).caps[0] = a;
    (*qr).caps[1] = b;
    (*qr).caps[2] = c;
    (*qr).align_region = -1i32;
    /* Rotate each capstone so that corner 0 is top-left with respect
     * to the grid.
     */
    i = 0i32;
    while i < 3i32 {
        let mut cap: *mut quirc_capstone = &mut *(*q)
            .capstones
            .as_mut_ptr()
            .offset(*(*qr).caps.as_mut_ptr().offset(i as isize) as isize)
            as *mut quirc_capstone;
        rotate_capstone(cap, &mut h0, &mut hd);
        (*cap).qr_grid = qr_index;
        i += 1
    }
    /* Check the timing pattern. This doesn't require a perspective
     * transform.
     */
    if !(measure_timing_pattern(q, qr_index) < 0i32) {
        /* Make an estimate based for the alignment pattern based on extending
         * lines from capstones A and C.
         */
        if !(line_intersect(
            &mut *(*(*q).capstones.as_mut_ptr().offset(a as isize))
                .corners
                .as_mut_ptr()
                .offset(0),
            &mut *(*(*q).capstones.as_mut_ptr().offset(a as isize))
                .corners
                .as_mut_ptr()
                .offset(1),
            &mut *(*(*q).capstones.as_mut_ptr().offset(c as isize))
                .corners
                .as_mut_ptr()
                .offset(0),
            &mut *(*(*q).capstones.as_mut_ptr().offset(c as isize))
                .corners
                .as_mut_ptr()
                .offset(3),
            &mut (*qr).align,
        ) == 0)
        {
            /* On V2+ grids, we should use the alignment pattern. */
            if (*qr).grid_size > 21i32 {
                /* Try to find the actual location of the alignment pattern. */
                find_alignment_pattern(q, qr_index);
                /* Find the point of the alignment pattern closest to the
                 * top-left of the QR grid.
                 */
                if (*qr).align_region >= 0i32 {
                    let mut psd: polygon_score_data = polygon_score_data {
                        ref_0: quirc_point { x: 0, y: 0 },
                        scores: [0; 4],
                        corners: 0 as *mut quirc_point,
                    };
                    let mut reg: *mut quirc_region = &mut *(*q)
                        .regions
                        .as_mut_ptr()
                        .offset((*qr).align_region as isize)
                        as *mut quirc_region;
                    /* Start from some point inside the alignment pattern */
                    memcpy(
                        &mut (*qr).align as *mut quirc_point as *mut libc::c_void,
                        &mut (*reg).seed as *mut quirc_point as *const libc::c_void,
                        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
                    );
                    memcpy(
                        &mut psd.ref_0 as *mut quirc_point as *mut libc::c_void,
                        &mut hd as *mut quirc_point as *const libc::c_void,
                        ::std::mem::size_of::<quirc_point>() as libc::c_ulong,
                    );
                    psd.corners = &mut (*qr).align;
                    psd.scores[0] = -hd.y * (*qr).align.x + hd.x * (*qr).align.y;
                    flood_fill_seed(
                        q,
                        (*reg).seed.x,
                        (*reg).seed.y,
                        (*qr).align_region,
                        1i32,
                        None,
                        0 as *mut libc::c_void,
                        0i32,
                    );
                    flood_fill_seed(
                        q,
                        (*reg).seed.x,
                        (*reg).seed.y,
                        1i32,
                        (*qr).align_region,
                        Some(
                            find_leftmost_to_line
                                as unsafe fn(
                                    _: *mut libc::c_void,
                                    _: libc::c_int,
                                    _: libc::c_int,
                                    _: libc::c_int,
                                ) -> (),
                        ),
                        &mut psd as *mut polygon_score_data as *mut libc::c_void,
                        0i32,
                    );
                }
            }
            setup_qr_perspective(q, qr_index);
            return;
        }
    }
    /* We've been unable to complete setup for this grid. Undo what we've
     * recorded and pretend it never happened.
     */
    i = 0i32;
    while i < 3i32 {
        (*q).capstones[(*qr).caps[i as usize] as usize].qr_grid = -1i32;
        i += 1
    }
    (*q).num_grids -= 1;
}
unsafe fn test_neighbours(
    mut q: *mut Quirc,
    mut i: libc::c_int,
    mut hlist: *const neighbour_list,
    mut vlist: *const neighbour_list,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut best_score: libc::c_double = 0.0f64;
    let mut best_h: libc::c_int = -1i32;
    let mut best_v: libc::c_int = -1i32;
    /* Test each possible grouping */
    j = 0i32;
    while j < (*hlist).count {
        k = 0i32;
        while k < (*vlist).count {
            let mut hn: *const neighbour =
                &*(*hlist).n.as_ptr().offset(j as isize) as *const neighbour;
            let mut vn: *const neighbour =
                &*(*vlist).n.as_ptr().offset(k as isize) as *const neighbour;
            let mut score: libc::c_double = fabs(1.0f64 - (*hn).distance / (*vn).distance);
            if !(score > 2.5f64) {
                if best_h < 0i32 || score < best_score {
                    best_h = (*hn).index;
                    best_v = (*vn).index;
                    best_score = score
                }
            }
            k += 1
        }
        j += 1
    }
    if best_h < 0i32 || best_v < 0i32 {
        return;
    }
    record_qr_grid(q, best_h, i, best_v);
}
unsafe fn test_grouping(mut q: *mut Quirc, mut i: libc::c_int) {
    let mut c1: *mut quirc_capstone =
        &mut *(*q).capstones.as_mut_ptr().offset(i as isize) as *mut quirc_capstone;
    let mut j: libc::c_int = 0;
    let mut hlist: neighbour_list = neighbour_list {
        n: [neighbour {
            index: 0,
            distance: 0.,
        }; 32],
        count: 0,
    };
    let mut vlist: neighbour_list = neighbour_list {
        n: [neighbour {
            index: 0,
            distance: 0.,
        }; 32],
        count: 0,
    };
    if (*c1).qr_grid >= 0i32 {
        return;
    }
    hlist.count = 0i32;
    vlist.count = 0i32;
    /* Look for potential neighbours by examining the relative gradients
     * from this capstone to others.
     */
    j = 0i32;
    while j < (*q).num_capstones {
        let mut c2: *mut quirc_capstone =
            &mut *(*q).capstones.as_mut_ptr().offset(j as isize) as *mut quirc_capstone;
        let mut u: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        if !(i == j || (*c2).qr_grid >= 0i32) {
            perspective_unmap((*c1).c.as_mut_ptr(), &mut (*c2).center, &mut u, &mut v);
            u = fabs(u - 3.5f64);
            v = fabs(v - 3.5f64);
            if u < 0.2f64 * v {
                let fresh5 = hlist.count;
                hlist.count = hlist.count + 1;
                let mut n: *mut neighbour =
                    &mut *hlist.n.as_mut_ptr().offset(fresh5 as isize) as *mut neighbour;
                (*n).index = j;
                (*n).distance = v
            }
            if v < 0.2f64 * u {
                let fresh6 = vlist.count;
                vlist.count = vlist.count + 1;
                let mut n_0: *mut neighbour =
                    &mut *vlist.n.as_mut_ptr().offset(fresh6 as isize) as *mut neighbour;
                (*n_0).index = j;
                (*n_0).distance = u
            }
        }
        j += 1
    }
    if !(hlist.count != 0 && vlist.count != 0) {
        return;
    }
    test_neighbours(q, i, &mut hlist, &mut vlist);
}
unsafe fn pixels_setup(mut q: *mut Quirc, mut threshold: uint8_t) {
    let mut source: *mut uint8_t = (*q).image;
    let mut dest: *mut quirc_pixel_t = (*q).pixels;
    let mut length: libc::c_int = (*q).w * (*q).h;
    loop {
        let fresh7 = length;
        length = length - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = source;
        source = source.offset(1);
        let mut value: uint8_t = *fresh8;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = if (value as libc::c_int) < threshold as libc::c_int {
            1i32
        } else {
            0i32
        } as quirc_pixel_t
    }
}
#[no_mangle]
pub unsafe fn quirc_begin(
    mut q: *mut Quirc,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) -> *mut uint8_t {
    (*q).num_regions = 2i32;
    (*q).num_capstones = 0i32;
    (*q).num_grids = 0i32;
    if !w.is_null() {
        *w = (*q).w
    }
    if !h.is_null() {
        *h = (*q).h
    }
    return (*q).image;
}
#[no_mangle]
pub unsafe fn quirc_end(mut q: *mut Quirc) {
    let mut i: libc::c_int = 0;
    let mut threshold: uint8_t = otsu(q);
    pixels_setup(q, threshold);
    i = 0i32;
    while i < (*q).h {
        finder_scan(q, i);
        i += 1
    }
    i = 0i32;
    while i < (*q).num_capstones {
        test_grouping(q, i);
        i += 1
    }
}
/* Extract the QR-code specified by the given index. */
#[no_mangle]
pub unsafe fn quirc_extract(
    mut q: *const Quirc,
    mut index: libc::c_int,
    mut code: *mut quirc_code,
) {
    let mut qr: *const quirc_grid =
        &*(*q).grids.as_ptr().offset(index as isize) as *const quirc_grid;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0i32;
    if index < 0i32 || index > (*q).num_grids {
        return;
    }
    memset(
        code as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<quirc_code>() as libc::c_ulong,
    );
    perspective_map(
        (*qr).c.as_ptr(),
        0.0f64,
        0.0f64,
        &mut *(*code).corners.as_mut_ptr().offset(0),
    );
    perspective_map(
        (*qr).c.as_ptr(),
        (*qr).grid_size as libc::c_double,
        0.0f64,
        &mut *(*code).corners.as_mut_ptr().offset(1),
    );
    perspective_map(
        (*qr).c.as_ptr(),
        (*qr).grid_size as libc::c_double,
        (*qr).grid_size as libc::c_double,
        &mut *(*code).corners.as_mut_ptr().offset(2),
    );
    perspective_map(
        (*qr).c.as_ptr(),
        0.0f64,
        (*qr).grid_size as libc::c_double,
        &mut *(*code).corners.as_mut_ptr().offset(3),
    );
    (*code).size = (*qr).grid_size;
    y = 0i32;
    while y < (*qr).grid_size {
        let mut x: libc::c_int = 0;
        x = 0i32;
        while x < (*qr).grid_size {
            if read_cell(q, index, x, y) > 0i32 {
                (*code).cell_bitmap[(i >> 3i32) as usize] =
                    ((*code).cell_bitmap[(i >> 3i32) as usize] as libc::c_int | 1i32 << (i & 7i32))
                        as uint8_t
            }
            i += 1;
            x += 1
        }
        y += 1
    }
}
